use config::colour_pal::pal_groups;
use dioxus::prelude::*;
use crate::PALETTES;

#[component]
pub fn ColourPalette() -> Element {
    let editing = use_signal(|| "default");

    rsx! {
        div {
            overflow: "visible",

            h3 { "Colour Palette" }
            select {
                for (name, palette) in PALETTES.read().iter() {
                    option {
                        "{name}"
                    }
                }
            }
            button {
                "Create New"
            }

            div {
                display: "flex",
                flex_direction: "row",
                for group in pal_groups() {
                    div {
                        display: "flex",
                        flex_direction: "column",
                        padding: "2%",
                        for name in group {
                            div {
                                class: "colorinput",
                                label { "{readable(name)}" }
                                input { 
                                    r#type: "color",
                                    value: PALETTES.read()[editing()][name].clone(),
                                    onchange: move |v| { 
                                        PALETTES.write().get_mut(editing()).unwrap().insert(name.to_string(), v.value());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn readable(text: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in text.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(' ');
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
