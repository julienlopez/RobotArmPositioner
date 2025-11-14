use dioxus::prelude::*;

use crate::logic::{Infra, State};

#[component]
pub fn Screen(infra : Infra, state: Signal<State>) -> Element {
    rsx! {
        div { "This is a screen component." }
        svg {

        }
    }
}