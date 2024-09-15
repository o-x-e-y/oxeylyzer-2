#[cfg(not(target_arch = "wasm32"))]
use oxeylyzer_core::prelude::{Analyzer, Layout};

#[cfg(not(target_arch = "wasm32"))]
pub fn analyzer_layout(corpus: &str, layout: &str) -> (Analyzer, Layout) {
    use oxeylyzer_core::{analyze::Analyzer, data::Data, layout::Layout};

    let data = Data::load(format!("./data/{corpus}.json")).expect("this should exist");

    let weights = oxeylyzer_core::weights::dummy_weights();

    let analyzer = Analyzer::new(data, weights);

    let layout = Layout::load(format!("./layouts/{layout}.dof"))
        .expect("this layout is valid and exists, soooo");

    (analyzer, layout)
}
