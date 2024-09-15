mod corpus_data;
mod create_keyboard;
mod settings;
mod tools;

use web_components::{
    layouts::{HeatmapData, LayoutsFolder},
    util::*,
    *,
};

use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    let heatmap_data = create_resource(
        move || "/public/heatmap_data.json".into(),
        util::load_json::<HeatmapData>,
    );

    let heatmap_theme = HeatmapTheme::default();

    let enable_heatmap = EnableHeatmap(create_rw_signal(true));

    let layout_names = LayoutNames(embedded_names::<LayoutsFolder>());

    let weights = GlobalWeights::default();

    provide_context(heatmap_data);
    provide_context(heatmap_theme);
    provide_context(enable_heatmap);
    provide_context(layout_names);
    provide_context(weights);

    let view_layouts = move || {
        let names = create_memo(move |_| expect_context::<LayoutNames>().0);
        view! {
            <div class="m-6">
                <layouts::LayoutLinks names></layouts::LayoutLinks>
            </div>
        }
    };

    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="bg-darker text-txt">
                <home::Navigation/>
                <Routes>
                    <Route path="/" view=home::Home/>
                    <Route path="/layouts" view=|| view! { <Outlet/> }>
                        <Route path="" view=view_layouts/>

                        <Route path=":name" view=analyze::RenderAnalyzer/>
                    </Route>
                    <Route path="/posts" view=|| view! { <Outlet/> }>
                        <Route path="" view=posts::RenderPostLinks/>
                        <Route path=":name" view=posts::RenderPost/>
                    </Route>
                    <Route path="/tools" view=|| view! { <Outlet/> }>
                        <Route path="" view=tools::Tools/>
                        <Route path="/generate-corpus-data" view=corpus_data::GenerateCorpusData/>
                        <Route path="/create-keyboard" view=create_keyboard::CreateKeyboard/>
                    </Route>
                    <Route path="/search/:query" view=search::QuerySearch/>
                    <Route path="/settings" view=settings::Settings/>
                </Routes>
            </main>
        </Router>
    }
}