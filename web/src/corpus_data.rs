use futures::stream::{self, StreamExt};
use leptos::*;
use oxeylyzer_core::prelude::{CleanCorpus, CorpusCleaner, Data, SHIFT_CHAR};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::FileList;

use crate::settings::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusInput {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusDataSettings {
    pub chars: RwSignal<String>,
    pub char_mappings: RwSignal<Vec<(char, char)>>,
    pub uppercase_mappings: RwSignal<Vec<(char, char)>>,
    pub multi_mappings: RwSignal<Vec<(char, String)>>,
    pub dead_key_mappings: RwSignal<Vec<(char, Vec<(char, char)>)>>,
    pub include_space: RwSignal<bool>,
    pub include_tab: RwSignal<bool>,
    pub include_enter: RwSignal<bool>,
    pub enable_repeat_key: RwSignal<bool>,
    pub shift_char: RwSignal<Option<char>>,
    pub uppercase_qwerty_punctuation: RwSignal<bool>,
    pub normalize_misc_punctuation: RwSignal<bool>,
}

impl Default for CorpusDataSettings {
    fn default() -> Self {
        Self {
            chars: create_rw_signal("abcdefghijklmnopqrstuvwxyz".chars().collect()),
            shift_char: create_rw_signal(Some(SHIFT_CHAR)),
            uppercase_qwerty_punctuation: create_rw_signal(true),
            include_space: create_rw_signal(true),
            include_tab: create_rw_signal(false),
            include_enter: create_rw_signal(false),
            enable_repeat_key: create_rw_signal(false),
            normalize_misc_punctuation: create_rw_signal(false),
            char_mappings: create_rw_signal(vec![]),
            uppercase_mappings: create_rw_signal(vec![]),
            multi_mappings: create_rw_signal(vec![]),
            dead_key_mappings: create_rw_signal(vec![]),
        }
    }
}

impl From<&CorpusDataSettings> for CorpusCleaner {
    fn from(settings: &CorpusDataSettings) -> Self {
        let special = match (
            settings.include_space.get_untracked(),
            settings.include_tab.get_untracked(),
            settings.include_enter.get_untracked()
        ) {
            (true, true, true) => " \t\n",
            (true, true, false) => " \t",
            (true, false, true) => " \n",
            (true, false, false) => " ",
            (false, true, true) => "\t\n",
            (false, true, false) => "\t",
            (false, false, true) => "\n",
            (false, false, false) => "",
        };

        CorpusCleaner::builder()
            .with_chars(settings.chars.get_untracked().chars())
            .with_chars(special.chars())
            .repeat_key(settings.enable_repeat_key.get_untracked())
            .build()
    }
}

async fn generate_data(files: Vec<CorpusInput>, settings: CorpusDataSettings) -> Data {
    let cleaner = CorpusCleaner::from(&settings);

    files
        .into_iter()
        .map(|f| f.text.chars().clean_corpus(&cleaner).collect::<Vec<_>>())
        .flatten()
        .collect()
}

async fn get_texts(files: Option<FileList>) -> Vec<CorpusInput> {
    let file_list = match files {
        Some(file_list) => file_list,
        None => {
            // logging::error!("Couldn't get files");
            return vec![];
        }
    };

    let length = file_list.length();

    let files = (0..length).flat_map(|i| match file_list.item(i) {
        Some(file) => Some(file),
        None => {
            logging::error!("Couldn't get item {i}");
            None
        }
    });

    stream::iter(files)
        .map(|file| async move {
            let text_future = JsFuture::from(file.text()).await;

            match text_future {
                Ok(text) => match text.as_string() {
                    Some(text) => {
                        let name = file.name();
                        Some(CorpusInput { name, text })
                    }
                    None => {
                        logging::error!("Couldn't convert file contents to string");
                        None
                    }
                },
                Err(e) => {
                    logging::error!("Couldn't get text from promise: {e:?}");
                    None
                }
            }
        })
        .buffer_unordered(10)
        .filter_map(|maybe_text| async move { maybe_text })
        .collect::<Vec<_>>()
        .await
}

#[component]
pub fn GenerateCorpusData() -> impl IntoView {
    let settings = CorpusDataSettings::default();
    provide_context(settings.clone());

    let (files, set_files) = create_signal(None);
    let (loaded_texts, set_loaded_texts) = create_signal(0usize);

    let data_action = create_action(|(texts, settings): &(Vec<CorpusInput>, CorpusDataSettings)| {
        let texts = texts.clone();
        let settings = settings.clone();

        async move {
            generate_data(texts, settings).await
        }
    });
    let data = data_action.value();

    let texts = create_blocking_resource(
        move || files(),
        move |files| {
            set_loaded_texts.update(|v| *v += 1);
            get_texts(files)
        },
    );

    let bytes = create_memo(move |_| {
        texts()
            .map(|texts| texts.iter().map(|text| text.text.len()).sum::<usize>())
            .unwrap_or_default()
    });

    let file_count = create_memo(move |_| texts().map(|texts| texts.len()).unwrap_or_default());

    let file_input = create_node_ref::<html::Input>();

    let on_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();

        match ev.data_transfer() {
            Some(dt) => set_files(dt.files()),
            None => logging::error!("Couldn't get data transfer from file drop"),
        }
    };

    let on_change = move |_| {
        if let Some(input) = file_input() {
            let files = input.files();
            set_files.set(files);
        };
    };

    let data_on_click = move |_| {
        let settings = settings.clone();

        match texts() {
            Some(texts) => data_action.dispatch((texts, settings)),
            None => {},
        }
    };

    let (construction, set_construction) = create_signal("block");
    let (construction_opacity, set_construction_opacity) = create_signal(7);

    view! {
        <div
            on:click=move |_| {
                set_construction_opacity.update(|v| *v -= 1);
                if construction_opacity() == 0 {
                    set_construction("none");
                }
            }
            style:display=construction
            style:opacity=move || (construction_opacity() as f64) / 7.0
            class="
            absolute top-1/3 -left-[10vw] w-[120vw] h-[24vw] rotate-12 bg-yellow-500
            text-black text-[8vw] select-none
            "
        >
            <p class="text-center align-middle leading-[22vw]">"Under\u{00A0}construction"</p>
        </div>
        <h2 class="text-3xl my-3 sm:my-4 text-center">"Generate Corpus Data"</h2>
        <div class="m-4 p-4 bg-black rounded-xl">
            <div class="sm:grid sm:grid-cols-[2fr_1fr_1fr] sm:gap-4">
                <label
                    on:dragover=move |ev| ev.prevent_default()
                    on:drop=on_drop
                    class="
                    inline-block w-full min-h-24 p-3 text-sm bg-white/5 rounded-lg border
                    border-ccc border-dashed hover:border-solid hover:border-blue-f"
                >
                    <input
                        on:change=on_change
                        node_ref=file_input
                        type="file"
                        multiple="multiple"
                        class="hidden"
                    />
                    {move || {
                        if loaded_texts() >= 2 {
                            view! { <ListFiles texts/> }.into_view()
                        } else {
                            view! {
                                <p class="text-base text-center text-txt/80">
                                    "Drop your corpora here or click to upload"
                                </p>
                            }
                                .into_view()
                        }
                    }}

                </label>
                <div class="w-fit px-3 my-3 sm:my-1">
                    <Suspense fallback=move || {}>
                        <p class="w-fit">"Bytes: " {bytes}</p>
                        <p class="w-fit">"Files: " {file_count}</p>
                    </Suspense>
                </div>
                <button
                    on:click=data_on_click
                    class="
                    w-full bg-white/5 hover:bg-white/10 text-lg text-txt/80
                    hover:text-txt/90 border border-header rounded-lg"
                >
                    <p class="text-center p-3">"Generate data"</p>
                </button>
            </div>
            <CorpusCleanerSettings>
                <SettingGroup header="Data">
                    <Suspense fallback=move || "Loading data...">
                        {move || {
                            data()
                                .map(|data| format!("{:#?}", data.chars))
                        }}
                    </Suspense>
                </SettingGroup>
            </CorpusCleanerSettings>
        </div>
    }
}

#[component]
fn ListFiles(texts: Resource<Option<FileList>, Vec<CorpusInput>>) -> impl IntoView {
    view! {
        <ul class="list-none grid gap-1">
            <Suspense fallback=move || {
                view! { <li class="text-center">"Loading corpus files..."</li> }
            }>
                {move || {
                    texts()
                        .map(|texts| {
                            texts
                                .iter()
                                .map(|text| {
                                    view! {
                                        <li class="overflow-x-hidden">
                                            <pre>{text.name.clone()}</pre>
                                        </li>
                                    }
                                })
                                .collect_view()
                        })
                }}

            </Suspense>
        </ul>
    }
}

#[component]
fn CorpusCleanerSettings(children: Children) -> impl IntoView {
    let CorpusDataSettings {
        chars,
        // uppercase_mappings,
        // char_mappings,
        // multi_mappings,
        // dead_key_mappings,
        include_space,
        include_tab,
        include_enter,
        enable_repeat_key,
        // shift_char,
        uppercase_qwerty_punctuation,
        normalize_misc_punctuation,
        ..
    } = expect_context::<CorpusDataSettings>();

    view! {
        <div class="mt-2 w-full grid grid-rows-2 gap-2 sm:mt-8 sm:grid-cols-2 sm:gap-4">
            <SettingGroup header="Configuration">
                <Setting description="Characters">
                    <CharsInput affect=chars/>
                </Setting>
                <Setting description="Include qwerty punctuation">
                    <CheckboxInput affect=uppercase_qwerty_punctuation/>
                </Setting>
                <Setting description="Include space">
                    <CheckboxInput affect=include_space/>
                </Setting>
                <Setting description="Include tab">
                    <CheckboxInput affect=include_tab/>
                </Setting>
                <Setting description="Include enter">
                    <CheckboxInput affect=include_enter/>
                </Setting>
                <Setting description="Enable repeat key">
                    <CheckboxInput affect=enable_repeat_key/>
                </Setting>
                <Setting description="Normalize misc punctuation">
                    <CheckboxInput affect=normalize_misc_punctuation/>
                </Setting>
            </SettingGroup>
            {children()}
        </div>
        // <div>
        //     <p>"chars: " {chars}</p>
        //     <p>"uppercase_mappings: " {uppercase_mappings}</p>
        //     <p>"char_mappings: " {char_mappings}</p>
        //     <p>"multi_mappings: " {multi_mappings}</p>
        //     <p>"dead_key_mappings: " {dead_key_mappings}</p>
        //     <p>"include_space: " {include_space}</p>
        //     <p>"include_tab: " {include_tab}</p>
        //     <p>"include_enter: " {include_enter}</p>
        //     <p>"enable_repeat_key: " {enable_repeat_key}</p>
        //     <p>"shift_char: " {shift_char}</p>
        //     <p>"uppercase_qwerty_punctuation: " {uppercase_qwerty_punctuation}</p>
        //     <p>"normalize_misc_punctuation: " {normalize_misc_punctuation}</p>
        // </div>
    }
}

#[component]
fn CharsInput(affect: RwSignal<String>) -> impl IntoView {
    view! {
        <textarea
            on:input=move |ev| {
                let value = event_target_value(&ev);
                if let Ok(val) = value.parse::<String>() {
                    affect.set(val);
                }
            }

            type="text"
            class="w-full max-w-96 px-1 resize-none bg-darker border-2 border-ccc/80 rounded-md"
        >
            {affect.get_untracked().to_string()}
        </textarea>
    }
}
