use leptos::{view, IntoView};
use leptos::leptos_dom::error;
use leptos::prelude::{signal, Action};
use leptos::wasm_bindgen::JsError;
use leptos::prelude::*;


fn Worker() -> impl IntoView {

    view! {
        <div>
            "test"
        </div>
    }
}


fn main() {

    leptos::mount::mount_to_body(Worker)
}
