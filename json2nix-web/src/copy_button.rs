use leptos::*;
use leptos_use::{UseClipboardReturn, use_clipboard};
use std::ops::Not;

#[component]
pub fn CopyButton(#[prop(into)] value: Signal<String>) -> impl IntoView {
    let UseClipboardReturn {
        is_supported,
        text: _,
        copied,
        copy,
    } = use_clipboard();

    view! {
        <button
            disabled=move || is_supported.get().not()
            on:click={
                let copy = copy.clone();
                move |_| copy(value.get().as_str())
            }
            class="button"
            title=move || if is_supported.get() { "Copy" } else { "Cannot copy! Your browser does not support the clipboard API!" }
        >
            <span class="icon">
                <Show
                    when=move || copied.get()
                    fallback=|| view!{
                        // Copy icon
                        <svg fill="var(--bulma-text-bold)" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            // Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.
                            <path d="M208 0H332.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H208c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128h80v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z"/>
                        </svg>
                    }
                >
                    // Tick icon
                    <svg fill="var(--bulma-success)" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                        // Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.
                        <path d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209L241 337c-9.4 9.4-24.6 9.4-33.9 0l-64-64c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l47 47L335 175c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9z"/>
                    </svg>
                </Show>
            </span>
        </button>
    }
}
