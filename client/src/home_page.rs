use crate::lobby::Lobby;
use crate::roguelike_racer_game::RoguelikeRacerGame;
use crate::websocket_provider::WebsocketProvider;
use common::adventuring_party::AdventuringParty;
use common::packets::server_to_client::{ClientGameListState, RoguelikeRacerAppState};
use leptos::*;

#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    provide_context(cx, create_rw_signal::<Option<AdventuringParty>>(cx, None));
    provide_context(cx, create_rw_signal(cx, ClientGameListState::new()));
    provide_context(
        cx,
        create_rw_signal::<RoguelikeRacerAppState>(cx, RoguelikeRacerAppState::new()),
    );

    let adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>(cx);
    view! { cx,
        <WebsocketProvider>
            <Show
                when=move || { adventuring_party.get().is_none() }
                fallback=|cx| view!{cx, <RoguelikeRacerGame/>}
            >
                <Lobby/>
            </Show>
        </WebsocketProvider>
    }
}
