use dioxus::prelude::*;

use crate::logic::{solve, Infra, State};

#[component]
pub fn Screen(infra: Infra, state: Signal<State>) -> Element {
    let state_read = state.read();

    // Calculate the positions of each joint and end point
    let mut points = vec![(300.0, 400.0)]; // Base position (origin)
    let mut current_x = 300.0;
    let mut current_y = 400.0;
    let mut cumulative_angle = 0.0;

    for (arm, angle) in infra.arms.iter().zip(state_read.angles.iter()) {
        cumulative_angle += angle;
        let dx = arm.length * cumulative_angle.cos();
        let dy = arm.length * cumulative_angle.sin();
        current_x += dx;
        current_y -= dy; // Subtract because SVG y-axis goes down
        points.push((current_x, current_y));
    }

    // Create line segments
    let lines = points.windows(2).map(|window| {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];
        rsx! {
            line {
                x1: "{x1}",
                y1: "{y1}",
                x2: "{x2}",
                y2: "{y2}",
                stroke: "blue",
                stroke_width: "4"
            }
        }
    });

    // Create circles at joints
    let joints = points.iter().enumerate().map(|(i, (x, y))| {
        let fill_color = if i == 0 { "red" } else if i == points.len() - 1 { "green" } else { "orange" };
        let radius = if i == 0 { "8" } else if i == points.len() - 1 { "6" } else { "5" };
        rsx! {
            circle {
                cx: "{x}",
                cy: "{y}",
                r: radius,
                fill: fill_color
            }
        }
    });

    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; justify-content: center; align-items: center;",
            svg {
                width: "800",
                height: "600",
                view_box: "0 0 600 600",
                style: "border: 1px solid #ccc; background-color: #f9f9f9; cursor: crosshair;",
                onclick: move |evt| {
                    let x = evt.data().page_coordinates().x;
                    let y = evt.data().page_coordinates().y;

                    // Call the solver with the clicked position
                    if let Some(new_angles) = solve(&infra, (x as f32, y as f32)) {
                        state.write().angles = new_angles;
                    }
                },

                // Draw grid for reference
                defs {
                    pattern {
                        id: "grid",
                        width: "50",
                        height: "50",
                        pattern_units: "userSpaceOnUse",
                        rect {
                            width: "50",
                            height: "50",
                            fill: "none"
                        }
                        path {
                            d: "M 50 0 L 0 0 0 50",
                            fill: "none",
                            stroke: "#e0e0e0",
                            stroke_width: "1"
                        }
                    }
                }
                rect {
                    width: "800",
                    height: "600",
                    fill: "url(#grid)"
                }

                // Draw arm segments
                {lines},

                // Draw joints
                {joints}
            }
        }
    }
}