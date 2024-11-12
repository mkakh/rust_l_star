pub(crate) mod dfa;
mod table;

use dfa::{Symbol, DFA};
use std::collections::{HashSet, VecDeque};
use table::ObservationTable;

fn concat(a: &[Symbol], b: &[Symbol]) -> Vec<Symbol> {
    if a == &["ε".to_string()] && b == &["ε".to_string()] {
        vec!["ε".to_string()]
    } else if a == &["ε".to_string()] {
        b.to_vec()
    } else if b == &["ε".to_string()] {
        a.to_vec()
    } else {
        let mut result = Vec::with_capacity(a.len() + b.len());
        result.extend_from_slice(a);
        result.extend_from_slice(b);
        result
    }
}

fn membership_query(target: &DFA, input: &[Symbol]) -> Result<bool, String> {
    target.run(input)
}

// returns a counter example
fn equivalence_query(target: &DFA, hypothesis: &DFA) -> Option<Vec<Symbol>> {
    let max_length = target.states_size() + 1;
    let mut queue = VecDeque::new();

    queue.push_back(vec!['ε'.to_string()]);

    while let Some(word) = queue.pop_front() {
        // Check if running the word on both DFAs results in the same state
        if target.run(&word) != hypothesis.run(&word) {
            eprintln!("target: {:?}", target);
            eprintln!("hypo: {:?}", hypothesis);
            return Some(word);
        }

        // If the current word's length is less than the max length, extend it
        if word.len() < max_length {
            for symbol in target.get_alphabet() {
                //let extended_word = concat(&word, &vec![symbol.clone()]);
                queue.push_back(concat(&word, &vec![symbol.clone()]));
            }
        }
    }

    // If no counterexample is found, return None
    None
}

fn construct_automaton(table: &ObservationTable, alphabet: &HashSet<Symbol>) -> DFA {
    let states = table.get_states().unwrap();
    let alphabet = alphabet.clone();
    let init_state = table.get_value_as_state(&vec!['ε'.to_string()]).unwrap();
    let mut transition_function = Vec::new();
    for prefix in table.get_rows().iter() {
        for a in alphabet.iter() {
            transition_function.push((
                (table.get_value_as_state(prefix).unwrap(), a.clone()),
                table
                    .get_value_as_state(&concat(prefix, &vec![a.clone()]))
                    .unwrap(),
            ));
        }
    }

    let mut final_states = vec![];

    assert!(table.get_columns().first() == Some(&vec!['ε'.to_string()]));
    for st in states.iter() {
        if let Some(i) = st.chars().next() {
            if i == '1' {
                final_states.push(st.clone());
            }
        }
    }

    DFA::new(
        transition_function,
        init_state,
        final_states.into_iter().collect(),
    )
}

fn fill(table: &mut ObservationTable, target: &DFA) {
    let mut unfilled_cell: Vec<(Vec<String>, Vec<String>)> = vec![];
    for row in table.get_rows().iter() {
        for column in table.get_columns().iter() {
            for a in target.get_alphabet().iter() {
                if !table.is_filled(row, column) {
                    unfilled_cell.push((row.clone(), column.clone()));
                }

                // This row can be inexistent in the table
                if !table.is_filled(&concat(row, &vec![a.to_owned()]), column) {
                    unfilled_cell.push((concat(row, &vec![a.to_owned()]), column.clone()));
                }
            }
        }
    }

    for (r, c) in unfilled_cell {
        table.fill_cell(&r, &c, membership_query(target, &concat(&r, &c)).unwrap());
    }
}

fn make_consistent(table: &mut ObservationTable, target: &DFA) {
    let mut is_consistent = false;
    while !is_consistent {
        is_consistent = true;
        'label: for s1 in table.get_rows().iter() {
            for s2 in table.get_rows().iter() {
                if table.get_value(s1) == table.get_value(s2) {
                    for a in target.get_alphabet().iter() {
                        for e in table.get_columns().iter() {
                            if table.get_cell(&concat(s1, &vec![a.to_owned()]), e)
                                != table.get_cell(&concat(s2, &vec![a.to_owned()]), e)
                            {
                                eprintln!("Making consistent");
                                is_consistent = false;
                                table.add_columns(concat(&vec![a.to_owned()], e));
                                fill(table, target);
                                eprintln!("{}", table);
                                break 'label;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn make_closed(table: &mut ObservationTable, target: &DFA) {
    let mut is_closed = false;
    let mut states = Vec::new();
    while !is_closed {
        states.clear();
        if let Ok(st) = table.get_states() {
            states = st;
        } else {
            fill(table, target);
            states = table.get_states().unwrap();
        }

        is_closed = true;
        'label: for s in table.get_rows().iter() {
            for a in target.get_alphabet().iter() {
                let sa = concat(s, &vec![a.to_owned()]);

                if !states.contains(&table.get_value_as_state(&sa).unwrap()) {
                    eprintln!("Making closed");
                    table.add_rows(sa);
                    fill(table, target);
                    is_closed = false;
                    eprintln!("{}", table);
                    break 'label;
                }
            }
        }
    }
}

pub fn learn(target: &DFA) -> (DFA, ObservationTable) {
    let mut table = ObservationTable::new();
    fill(&mut table, target);
    eprintln!("{}", table);

    loop {
        make_consistent(&mut table, target);
        make_closed(&mut table, target);

        if let Some(ce) =
            equivalence_query(target, &construct_automaton(&table, target.get_alphabet()))
        {
            eprintln!("Counter example found: {:?}", ce);
            // add all prefix of the counter example to rows
            for i in 1..=ce.len() {
                table.add_rows(ce[..i].to_vec());
            }
            fill(&mut table, target);
            eprintln!("{}", table);
        } else {
            break;
        }
    }

    (construct_automaton(&table, target.get_alphabet()), table)
}

mod tests {
    #[test]
    fn test_l_star() {
        use super::{equivalence_query, learn, DFA};
        let target = DFA::new(
            vec![
                (("[ε]", "a"), "[ε]"),
                (("[ε]", "b"), "[b]"),
                (("[b]", "a"), "[ba]"),
                (("[b]", "b"), "[b]"),
                (("[ba]", "a"), "[ba]"),
                (("[ba]", "b"), "[ba]"),
            ],
            "[ε]",
            vec!["[b]"],
        );

        let (dfa, _table) = learn(&target);
        println!("DFA: {:?}", dfa);
        assert_eq!(equivalence_query(&target, &dfa), None);
    }
}
