pub mod app;
use cfg_if::cfg_if;
pub mod common_components;
pub mod game_setup;
pub mod home_page;
pub mod lobby;
pub mod websocket_provider;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}
