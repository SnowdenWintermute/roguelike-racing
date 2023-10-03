use crate::lobby::Lobby;
// use crate::roguelike_racer_game::RoguelikeRacerGame;
use crate::websocket_provider::WebsocketProvider;
use common::adventuring_party::AdventuringParty;
use common::packets::server_to_client::{ClientGameListState, RoguelikeRacerAppState};
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    provide_context(create_rw_signal::<Option<AdventuringParty>>(None));
    provide_context(create_rw_signal(ClientGameListState::new()));
    provide_context(create_rw_signal::<RoguelikeRacerAppState>(
        RoguelikeRacerAppState::new(),
    ));
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
