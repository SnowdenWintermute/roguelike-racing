pub mod action_menu;
pub mod combat_log;
pub mod combatant_detail_tab;
pub mod dungeon_room;
pub mod tabbed_display;
use crate::{
    components::game::{
        action_menu::ActionMenu, dungeon_room::DungeonRoom, tabbed_display::TabbedDisplay,
    },
    store::game_store::GameStore,
};
use gloo::{console::log, events::EventListener};
use gloo_utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Game)]
pub fn game() -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let game = game_state
        .game
        .clone()
        .expect("component only shown if game exists");

    let click_listener_state = use_state(|| None::<EventListener>);
    let cloned_dispatch = game_dispatch.clone();
    use_effect_with((), move |_| {
        let listener = EventListener::new(&window(), "click", move |event| {
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let target = event.target();
            if let Some(target) = target {
                let element = target.unchecked_into::<HtmlElement>();
                log!(&format!("{:#?}", element.id()));
                let id_tag = element.id().split("-").collect::<Vec<&str>>()[0].to_string();
                if id_tag != "combatant".to_string() {
                    cloned_dispatch.reduce_mut(|store| store.detailed_entity = None);
                };
            }
        });
        click_listener_state.set(Some(listener));
    });

    let cloned_dispatch = game_dispatch.clone();
    let keyup_listener_state = use_state(|| None::<EventListener>);
    use_effect_with((), move |_| {
        let listener = EventListener::new(&window(), "keyup", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            if event.key() == "Escape" {
                cloned_dispatch.reduce_mut(|store| store.detailed_entity = None);
            }
        });
        keyup_listener_state.set(Some(listener));
    });

    html!(
        <main class="h-screen w-screen p-4 bg-gray-600 text-zinc-300 flex flex-col">
            <DungeonRoom game={game} party_id={game_state.current_party_id.expect("must have party id")} />
            <div class="flex h-1/2 max-h-[453px]" >
                <ActionMenu />
                <TabbedDisplay />
            </div>
        </main>
    )
}
