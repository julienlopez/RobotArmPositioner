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
        h2 { "Angles" }
        for (i, angle) in state.read().angles.iter().enumerate() {
            div {
                style: "margin: 10px 0; display: flex; align-items: center; gap: 10px;",
                label {
                    style: "min-width: 80px;",
                    "Joint {i + 1}:"
                }
                input {
                    r#type: "number",
                    value: "{angle}",
                    step: "0.1",
                    style: "width: 100px;",
                    oninput: move |evt| {
                        if let Ok(new_angle) = evt.value().parse::<f32>() {
                            state.write().angles[i] = new_angle;
                        }
                    }
                }
                span {
                    style: "color: #666; font-size: 0.9em;",
                    "rad"
                }
            }
        }
    }
}