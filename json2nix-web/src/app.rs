use crate::copy_button::CopyButton;
use crate::number_input::NumberInput;
use json2nix::json2nix;
use leptos::*;
use leptos_use::{use_cookie_with_options, utils::FromToStringCodec, UseCookieOptions};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn highlight_nix_code(code: &str) -> String;
}

const PROJECT_REPOSITORY_URL: &'static str = env!("CARGO_PKG_REPOSITORY");
const ONE_YEAR_IN_MILLISECONDS: i64 = 365 * 24 * 60 * 60 * 1000;

#[component]
pub fn App() -> impl IntoView {
    let (cookie_raw_input, set_cookie_raw_input) = use_cookie_with_options::<String, FromToStringCodec>(
        "raw_input",
        // TODO: set the same_site attribute when the SameSite struct is re-exported in a new version of leptos-use
        UseCookieOptions::default().max_age(ONE_YEAR_IN_MILLISECONDS),
    );
    let raw_input = RwSignal::new("".to_string());
    let initial_indentation = RwSignal::new(0);
    let indentation = RwSignal::new(2);

    let generated_nix_code_result = Signal::derive(move || {
        let json = raw_input.get();

        if json.trim().is_empty() {
            return Ok("".to_string());
        }

        json2nix(&json, initial_indentation.get(), indentation.get())
    });

    let nix_code_or_empty = Signal::derive(move || generated_nix_code_result.get().unwrap_or_default());

    let highlighted_nix_code = Signal::derive(move || {
        let generated_nix_code = generated_nix_code_result.get();

        match generated_nix_code {
            Ok(code) => Ok(highlight_nix_code(&code)),
            Err(err) => Err(err),
        }
    });

    let input_raw_input = move |event| {
        let input = event_target_value(&event);
        raw_input.set(input.clone());
        set_cookie_raw_input.set(Some(input));
    };

    view! {
        <div class="is-flex is-flex-direction-column m-0 p-5" style="height: 100vh;">
            <div class="columns">
                <div class="column is-flex is-align-items-center">
                    <div class="title" style="min-width: 20rem;">
                        "Convert JSON to Nix"
                    </div>
                </div>
                <div class="column">
                    <div class="columns">
                        <div class="column is-three-quarters is-flex is-justify-content-start">
                            <div class="mr-3">
                                <NumberInput
                                    id="initial_indentation"
                                    value=initial_indentation
                                    min=0
                                    max=50
                                    label="Left Margin".to_string()
                                />
                            </div>
                            <div class="ml-3">
                                <NumberInput
                                    id="indentation"
                                    value=indentation
                                    min=0
                                    max=50
                                    label="Indentation".to_string()
                                />
                            </div>
                        </div>
                        <div class="column is-flex is-justify-content-end is-align-items-center">
                            <div>
                                <a class="button" href=PROJECT_REPOSITORY_URL target="_blank" rel="noopener noreferrer" title="Source code repository">
                                    <span class="icon">
                                        <svg fill="var(--bulma-text-bold)" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512">
                                            // Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.
                                            <path d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3 .3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5 .3-6.2 2.3zm44.2-1.7c-2.9 .7-4.9 2.6-4.6 4.9 .3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3 .7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3 .3 2.9 2.3 3.9 1.6 1 3.6 .7 4.3-.7 .7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3 .7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3 .7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/>
                                        </svg>
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="columns" style="height: 100%;">
                <div class="column is-half">
                    <textarea
                        on:input=input_raw_input
                        placeholder="Your JSON goes here..."
                        class=move || "textarea is-family-monospace is-size-6".to_string() + if highlighted_nix_code.get().is_err() { " is-danger" } else { "" }
                        style="height: 100%; resize: none; background-color: var(--bulma-pre-background); padding: var(--bulma-pre-padding);"
                        spellcheck="false"
                        wrap="off"
                        rows=10
                    >
                        // just for the initial value
                        {match cookie_raw_input.get_untracked() {
                            Some(initial_raw_input) => {
                                raw_input.set(initial_raw_input.clone());
                                initial_raw_input
                            },
                            None => "".to_string(),
                        }}
                    </textarea>
                </div>
                <div class="column is-half">
                    <div style="height: 100%; position: relative;">
                        <span style="position: absolute; right: 0; top: 0; margin: var(--bulma-pre-padding);">
                            <CopyButton value=nix_code_or_empty/>
                        </span>
                        <pre style="height: 100%;" class="is-size-6">
                            <code
                                inner_html=move || {
                                    match highlighted_nix_code.get() {
                                        Ok(code) => code,
                                        Err(err) => err,
                                    }
                                }
                            />
                        </pre>
                    </div>
                </div>
            </div>
        </div>
    }
}
