use crate::game_setup::GameSetup;
use crate::lobby::Lobby;
use crate::websocket_provider::WebsocketProvider;
use common::adventuring_party::AdventuringParty;
use common::app_consts::MAIN_CHAT_ROOM;
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::{ClientGameListState, RoomState};
use leptos::*;

#[derive(Clone)]
pub struct ClientPartyId(pub Option<u32>);

#[component]
pub fn HomePage() -> impl IntoView {
    provide_context(create_rw_signal::<Option<RoguelikeRacerGame>>(None));
    provide_context(create_rw_signal::<ClientPartyId>(ClientPartyId(None)));
    provide_context(create_rw_signal::<Option<AdventuringParty>>(None));
    provide_context(create_rw_signal(ClientGameListState::new()));
    provide_context(create_rw_signal(RoomState {
        room_name: MAIN_CHAT_ROOM.to_string(),
        users: Vec::new(),
    }));
    // let adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>();
    let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();

    view! {
        <WebsocketProvider>
            <Show when=move || { game.get().is_some() } fallback=|| view! { <Lobby/> }>
                <GameSetup/>
            </Show>
        </WebsocketProvider>
    }
}
