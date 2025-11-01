mod app;
mod checkbox;
mod copy_button;
mod number_input;

use leptos::prelude::mount_to_body;

fn main() {
    mount_to_body(|| app::App());
}
