use wasm_bindgen::prelude::*;

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
