use common::combat::magical_elements::MagicalElements;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use web_sys::HtmlElement;

// Import the JavaScript Date object
#[wasm_bindgen]
extern "C" {
    type Date;

    #[wasm_bindgen(static_method_of = Date)]
    fn now() -> f64;
}

// Function to get the current time in milliseconds since 1970
#[wasm_bindgen]
pub fn get_current_time() -> f64 {
    Date::now()
}

pub fn get_magical_element_tailwind_color(element: &MagicalElements) -> String {
    let str = match element {
        MagicalElements::Fire => "bg-firered",
        MagicalElements::Ice => "bg-iceblue",
        MagicalElements::Lightning => "bg-lightningpurple",
        MagicalElements::Water => "bg-waterblue",
        MagicalElements::Earth => "bg-earthyellow text-slate-700",
        MagicalElements::Wind => "bg-windgreen text-slate-700",
        MagicalElements::Dark => "bg-darknessblack",
        MagicalElements::Light => "bg-lightwhite text-slate-700",
    };
    str.to_string()
}

pub fn hide_bevy_canvas() {
    let bevy_canvas_node = window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .get_element_by_id("bevy")
        .expect("to have the bevy canvas")
        .unchecked_into::<HtmlElement>();
    bevy_canvas_node.set_class_name("hidden");
}
