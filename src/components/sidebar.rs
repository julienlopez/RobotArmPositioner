use dioxus::prelude::*;

use crate::logic::{Infra, State};

#[component]
pub fn SideBar(infra : Infra, state: Signal<State>) -> Element {
    rsx! {
        div { 
            id: "sidebar",
            InfraBox { infra: infra.clone() }
            StateBox { state: state }
        }
    }
}

#[component]
pub fn InfraBox(infra : Infra) -> Element {
    rsx! {
        h2 { "Arms" }
        for arm in infra.arms.iter() {
            div {
                h3 { "{arm.length}" }
            }
        }
    }
}
#[component]
pub fn StateBox(state: Signal<State>) -> Element {
    rsx! {
        h2 { "State" }
        for angle in state.read().angles.iter() {
            div {
                h3 { "{angle}" }
            }
        }
    }
}