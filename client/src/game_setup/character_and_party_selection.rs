#[allow(unused_imports)]
use crate::{
    common_components::button_basic::ButtonBasic,
    websocket_provider::send_client_input::send_client_input,
};
use common::game::{player_actions::PlayerInputs, RoguelikeRacerGame};
use leptos::*;
use web_sys::{MouseEvent, WebSocket};

#[component]
pub fn character_and_party_selection() -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();
    let game_name = move || {
        game.get()
            .expect("if this component is showing, a game should exist")
            .name
    };

    view! {
        <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400" id="game_list" >
            <h2>"Game: "{game_name}</h2>
            // partyless_players: HashMap::new(),
            // pub struct RoguelikeRacerPlayer {
            // pub actor_id: Option<usize>,
            // pub username: String,
            // pub character_ids: Option<Vec<u32>>,
            // pub ready: bool,
            // }

           // adventuring_parties: HashMap::new(),
             // pub struct AdventuringParty {
             //    pub id: u32,
             //    pub players: HashMap<String, RoguelikeRacerPlayer>,
             //    pub player_characters: HashMap<u32, Character>,

        </section>
    }
}
