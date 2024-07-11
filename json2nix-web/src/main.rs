mod app;
mod number_input;

use leptos::mount_to_body;

fn main() {
    mount_to_body(|| app::App());
}
