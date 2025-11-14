use dioxus::prelude::*;

#[component]
pub fn Screen() -> Element {
    rsx! {
        div { "This is a screen component." }
    }
}