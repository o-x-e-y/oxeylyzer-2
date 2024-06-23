use oxeylyzer_repl::*;

pub fn main() {
    let mut repl =
        Repl::with_config("./analyzer-config.toml").expect("Could not initialize repl: ");

    repl.run().expect("Encountered error: ")
}

#[test]
fn thing() {
    let vs = [15, 36, 54, 55, 25, 25, 55, 48, 36, 15];

    let max = vs.into_iter().max().unwrap_or_default();

    let s = max * 100;

    for v in vs {
        println!("{}", s / v);
    }
}
