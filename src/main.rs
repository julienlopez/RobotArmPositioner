use dioxus::prelude::*;

mod components;
mod views;
mod logic;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

use crate::logic::{Arm, Infra, State};

#[component]
fn App() -> Element {
    let infra = Infra{ arms: vec![Arm{length: 10.}, Arm{length: 15.}, Arm{ length: 5.}] };
    let state = use_signal(|| State::new(infra.arms.len()));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        views::Home{ infra: infra.clone(), state }
    }
}
