use oxeylyzer_repl::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let mut repl =
        Repl::with_config("./analyzer-config.toml").expect("Could not initialize repl: ");

    repl.run().expect("Encountered error: ")
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    println!("Repl doesn't currently have wasm support (what are you doing?");
}
