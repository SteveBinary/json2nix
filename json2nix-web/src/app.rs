use json2nix::json2nix;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());

    let set_json = move |event| set_value.set(event_target_value(&event));

    view! {
        <div>
            <textarea type="text" on:input=set_json></textarea>
        </div>
        <div>
            <pre>{move || json2nix(&value.get(), 0, 2).unwrap_or("error".to_string())}</pre>
        </div>
    }
}
