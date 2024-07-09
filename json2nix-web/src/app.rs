use crate::number_input::NumberInput;
use json2nix::json2nix;
use leptos::prelude::*;
use tachys::dom::event_target_value;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn highlight_nix_code(code: &str) -> String;
}

#[component]
pub fn App() -> impl IntoView {
    let (raw_input, set_raw_input) = signal("".to_string());
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

    let input_raw_input = move |event| set_raw_input.set(event_target_value(&event));

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
                    />
                </div>
                <div class="column is-half">
                    <pre style="height: 100%" class="is-size-6">
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
    }
}
