use leptos::*;

#[component]
pub fn NumberInput(#[prop(into)] value: RwSignal<usize>, min: usize, max: usize, id: &'static str, label: String) -> impl IntoView {
    let input_value = move |event| {
        let new_value = event_target_value(&event);
        if new_value.is_empty() {
            value.set(min);
        }
        match new_value.parse::<usize>() {
            Ok(new_value) => value.set(new_value.clamp(min, max)),
            Err(_) => {}
        };
    };

    view! {
        <div>
            <div class="field">
                <div class="field-label" style="width: 100%;">
                    <label for=id class="label has-text-left">{label}</label>
                </div>
                <div class="field-body">
                    <div class="field">
                        <p class="control">
                            <input
                                id=id
                                type="number"
                                min=min
                                max=max
                                on:input=input_value
                                prop:value=value
                                pattern="\\d*"
                                class="input"
                            />
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
