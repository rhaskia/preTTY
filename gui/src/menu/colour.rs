use config::colour_pal::pal_groups;
use dioxus::prelude::*;
use crate::PALETTES;

#[component]
pub fn ColourPalette() -> Element {
    let mut editing = use_signal(|| "default".to_string());
    let mut new_pal_name = use_signal(String::new);

    rsx! {
        div {
            overflow: "visible",

            h3 { "Colour Palette" }
            select {
                id: "paletteselect",
                value: "default",
                onmounted: |_| {
                    eval(r#"document.getElementById("paletteselect").value = "default""#);
                },
                onchange: move |v| {  
                    println!("{:?}", v.value());
                    editing.set(v.value());
                },
                for (name, _) in PALETTES.read().iter() {
                    option {
                        value: "{name}",
                        "{name}"
                    }
                }
            }

            input {
                placeholder: "Palette Name",
                onchange: move |v| new_pal_name.set(v.value()), 
            }

            button {
                onclick: move |_| {
                    if new_pal_name.read().is_empty() { return; }
                    PALETTES.write().insert(new_pal_name(), config::default_pal());
                    editing.set(new_pal_name());
                    eval(&format!("document.getElementById(\"paletteselect\").value = \"{}\"", new_pal_name()));
                },
                "Create New"
            }

            button {
                onclick: move |_| {
                    PALETTES.write().remove(editing());
                    // TODO popup confirm
                }
                "Delete Palette"
            }

            // TODO: flatten out
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
                                    value: PALETTES.read()[&editing()][name].clone(),
                                    onchange: move |v| { 
                                        PALETTES.write().get_mut(&editing()).unwrap().insert(name.to_string(), v.value());
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
