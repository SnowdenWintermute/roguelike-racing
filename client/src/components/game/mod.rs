mod action_menu;
mod character_sheet;
mod combat_log;
mod combatant_detail_tab;
mod dungeon_room;
mod tabbed_display;
use crate::{
    components::game::{
        action_menu::ActionMenu, character_sheet::CharacterSheet, dungeon_room::DungeonRoom,
        tabbed_display::TabbedDisplay,
    },
    store::{game_store::GameStore, lobby_store::LobbyStore},
};
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Game)]
pub fn game() -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let game = game_state
        .game
        .clone()
        .expect("component only shown if game exists");
    let player = game
        .players
        .get(&lobby_state.username)
        .expect("a player should exist by the username stored on the client")
        .clone();

    let party_id = game_state.current_party_id.expect("must have party id");

    let party = game
        .adventuring_parties
        .get(&party_id)
        .expect("must have a party id")
        .clone();

    // let click_listener_state = use_state(|| None::<EventListener>);
    // let cloned_dispatch = game_dispatch.clone();
    // use_effect_with((), move |_| {
    //     let listener = EventListener::new(&window(), "click", move |event| {
    //         let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
    //         let target = event.target();
    //         if let Some(target) = target {
    //             let element = target.unchecked_into::<HtmlElement>();
    //             let id_tag = element.id().split("-").collect::<Vec<&str>>()[0].to_string();
    //             // if id_tag != "combatant".to_string() {
    //             //     cloned_dispatch.reduce_mut(|store| store.detailed_entity = None);
    //             // };
    //         }
    //     });
    //     click_listener_state.set(Some(listener));
    // });

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

    let cloned_dispatch = game_dispatch.clone();
    use_effect_with((), move |_| {
        cloned_dispatch.reduce_mut(|game_state| {
            if let Some(ids) = &player.character_ids {
                let mut character_ids_vec = Vec::from_iter(ids);
                character_ids_vec.sort();
                game_state.focused_character_id = *character_ids_vec[0];
            }
        })
    });

    let focused_character = party.characters.get(&game_state.focused_character_id);

    html!(
        <main class="h-screen w-screen p-4 bg-gray-600 text-zinc-300 flex flex-col">
            <div class="h-1/2 flex mb-4" >
                <DungeonRoom game={game} party_id={party_id} />
                if game_state.viewing_inventory && focused_character.is_some(){
                    <CharacterSheet character={focused_character.as_deref().expect("is_some checked").clone()} />
                }
            </div>
            <div class="flex h-1/2 max-h-[453px]" >
                <ActionMenu adventuring_party={party.clone()} />
                <TabbedDisplay />
            </div>
        </main>
    )
}
