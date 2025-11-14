use dioxus::prelude::*;

use crate::logic::{Infra, State};

#[component]
pub fn Home(infra: Infra, state: Signal<State>) -> Element {
    rsx! {
        div {
            id: "home",
            h1 { "Robot Arm Positioner" }
            div {
                class: "layout",
                crate::components::SideBar { infra: infra.clone(), state }
                crate::components::Screen {infra: infra, state }
            }
        }
    }
}