mod app;
mod copy_button;
mod number_input;

use leptos::mount_to_body;

fn main() {
    mount_to_body(|| app::App());
}
