use std::collections::{HashMap, HashSet};

pub(crate) type Symbol = String;
pub(crate) type State = String;

#[derive(Debug)]
pub(crate) struct DFA {
    states: HashSet<State>,
    alphabet: HashSet<Symbol>,
    init_state: State,
    transitions: HashMap<(State, Symbol), State>,
    final_states: HashSet<State>,
}

impl DFA {
    pub(crate) fn new<S: Into<String>>(
        transitions: Vec<((S, S), S)>,
        init_state: S,
        final_states: Vec<S>,
    ) -> Self {
        let mut transition_table = HashMap::new();

        for ((state, symbol), next_state) in transitions {
            transition_table.insert((state.into(), symbol.into()), next_state.into());
        }

        let init_state = init_state.into();

        // Collect all states from transitions
        let mut states = HashSet::new();
        for (key, value) in &transition_table {
            states.insert(key.0.clone()); // the current state
            states.insert(value.clone()); // the next state
        }

        // Ensure the initial state is part of the states set
        states.insert(init_state.clone());

        // Collect all symbols from transitions
        let mut alphabet = HashSet::new();
        for (_, a) in transition_table.keys() {
            alphabet.insert(a.clone());
        }

        let final_states: HashSet<_> = final_states.into_iter().map(|s| s.into()).collect();

        DFA {
            states,
            alphabet,
            init_state,
            transitions: transition_table,
            final_states,
        }
    }

    pub(crate) fn run(&self, input: &[Symbol]) -> Result<bool, String> {
        let mut state = &self.init_state;
        for i in input.iter() {
            if !self.alphabet.contains(i) && i != "ε" {
                Err(format!("Symbol '{}' not in alphabet", i))?;
            }

            if i == "ε" {
                continue;
            } else {
                state = self.transitions.get(&(state.clone(), i.clone())).unwrap();
            }
        }

        Ok(self.final_states.contains(state))
    }

    pub(crate) fn get_alphabet(&self) -> &HashSet<Symbol> {
        &self.alphabet
    }

    pub(crate) fn states_size(&self) -> usize {
        self.states.len()
    }

    pub(crate) fn to_dot(&self) -> String {
        let mut dot_representation = String::from("digraph DFA {\n");
        dot_representation.push_str("    rankdir=LR;\n");
        dot_representation.push_str("    size=\"8,5\";\n");
        dot_representation.push_str("    node [shape = doublecircle]; ");
        for final_state in &self.final_states {
            dot_representation.push_str(&format!("{} ", final_state));
        }
        dot_representation.push_str(";\n");
        dot_representation.push_str("    node [shape = circle];\n");

        dot_representation.push_str(&format!(
            "    start [shape = point];\n    start -> {}\n",
            self.init_state
        ));

        for ((state, input), next_state) in &self.transitions {
            dot_representation.push_str(&format!(
                "    {} -> {} [label=\"{}\"];\n",
                state, next_state, input
            ));
        }

        dot_representation.push_str("}");
        dot_representation
    }
}
