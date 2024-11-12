mod lstar;

use lstar::{dfa::DFA, learn};
fn main() {
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
    println!("{}", dfa.to_dot());
}
