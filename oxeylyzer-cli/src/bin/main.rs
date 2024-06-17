use oxeylyzer_cli::*;

pub fn main() {
    // let data = Data::load("./data/shai.json")?;
    // let weights = Weights {
    //     heatmap: -1,
    //     sfbs: -60000,
    //     sfs: -20000,
    // };

    // let a = Analyzer::new(data, weights);

    // let layout1 = Layout::load("./layouts/rstn-oxey.dof")?;
    // let layout2 = Layout::load("./layouts/colemak-dh.dof")?;
    // let layout3 = Layout::load("./layouts/sturdy.dof")?;

    // let layouts = HashMap::from_iter([
    //     ("rstn-oxey".into(), layout1),
    //     ("colemak-dh".into(), layout2),
    //     ("sturdy".into(), layout3),
    // ]);

    // let repl = Repl::new(a, layouts);

    let mut repl =
        Repl::with_config("./analyzer-config.toml").expect("Could not initialize repl: ");

    repl.run().expect("Encountered error: ")
}
