use crate::number_input::NumberInput;
use json2nix::json2nix;
use leptos::*;
use leptos_use::{use_cookie, utils::FromToStringCodec};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn highlight_nix_code(code: &str) -> String;
}

#[component]
pub fn CopyButton() -> impl IntoView {
    view! {
        <button class="button">
            <span class="icon">
                <svg fill="var(--bulma-primary-on-scheme)" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                    // Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.
                    <path d="M208 0H332.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H208c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128h80v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z"/>
                </svg>
            </span>
        </button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (cookie_raw_input, set_cookie_raw_input) =
        use_cookie::<String, FromToStringCodec>("raw_input");
    let (raw_input, set_raw_input) = create_signal("".to_string());
    let initial_indentation = RwSignal::new(0);
    let indentation = RwSignal::new(2);

    let highlighted_nix_code = Signal::derive(move || {
        let json = raw_input.get();

        if json.trim().is_empty() {
            return Ok("".to_string());
        }

        let generated_nix_code = json2nix(&json, initial_indentation.get(), indentation.get());

        match generated_nix_code {
            Ok(code) => Ok(highlight_nix_code(&code)),
            Err(err) => Err(err),
        }
    });

    let input_raw_input = move |event| {
        let input = event_target_value(&event);
        set_raw_input.set(input.clone());
        set_cookie_raw_input.set(Some(input));
    };

    view! {
        <div class="is-flex is-flex-direction-column m-0 p-5" style="height: 100vh;">
            <div class="columns">
                <div class="column">
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
                                    label="Initial Indentation".to_string()
                                />
                            </div>
                            <div class="ml-3">
                                <NumberInput
                                    id="indentation"
                                    value=indentation
                                    min=0
                                    max=50
                                    label="Indentation Level".to_string()
                                />
                            </div>
                        </div>
                        <div class="column is-flex is-justify-content-end is-align-items-center">
                            <div>
                                "right"
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
                        {match cookie_raw_input.get_untracked() {
                            Some(initial_raw_input) => {
                                set_raw_input.set(initial_raw_input.clone());
                                initial_raw_input
                            },
                            None => "".to_string(),
                        }}
                    </textarea>
                </div>
                <div class="column is-half">
                    <div style="height: 100%; position: relative">
                        <span style="position: absolute; right: 0; top: 0; margin: var(--bulma-pre-padding);">
                            <CopyButton/>
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
