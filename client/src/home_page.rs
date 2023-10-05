use crate::lobby::Lobby;
use crate::websocket_provider::WebsocketProvider;
use common::adventuring_party::AdventuringParty;
use common::consts::MAIN_CHAT_ROOM;
use common::packets::server_to_client::{ClientGameListState, RoomState};
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    provide_context(create_rw_signal::<Option<AdventuringParty>>(None));
    provide_context(create_rw_signal(ClientGameListState::new()));
    provide_context(create_rw_signal(RoomState {
        room_name: MAIN_CHAT_ROOM.to_string(),
        users: Vec::new(),
    }));
    let adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>();

    view! {
        <WebsocketProvider>
            <Show
                when=move || { adventuring_party.get().is_none() }
                fallback=|| view!{"game component"}
            >
                <Lobby/>
            </Show>
        </WebsocketProvider>
    }
}
