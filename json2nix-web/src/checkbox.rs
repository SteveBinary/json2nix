use leptos::prelude::*;

#[component]
pub fn CheckBox(#[prop(into)] checked: RwSignal<bool>, id: &'static str, label: String) -> impl IntoView {
    view! {
        <div style="height: 100%;">
            <div class="field">
                <div class="field-label" style="width: 100%;">
                    <label for=id class="label has-text-centered">{label}</label>
                </div>
                <div class="field-body" style="height: var(--bulma-control-height);">
                    <div class="field is-flex is-justify-content-center is-align-content-center">
                        <input
                            id=id
                            type="checkbox"
                            on:input=move |event| checked.set(event_target_checked(&event))
                            prop:checked=checked
                            class="checkbox"
                            style="width: 1.5em;"
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
