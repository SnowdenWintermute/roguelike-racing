use crate::lobby::Lobby;
use crate::roguelike_racer_game::RoguelikeRacerGame;
use crate::websocket_provider::WebsocketProvider;
use common::adventuring_party::AdventuringParty;
use common::packets::server_to_client::RoguelikeRacerAppState;
use leptos::*;

#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    provide_context(
        cx,
        create_rw_signal::<Option<AdventuringParty>>(cx, Some(AdventuringParty::new(9))),
    );
    provide_context(
        cx,
        create_rw_signal::<Option<RoguelikeRacerAppState>>(cx, None),
    );

    let adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>(cx);
    view! { cx,
        <WebsocketProvider>
            <Show
                when=move || { adventuring_party.get().is_some() }
                fallback=|cx| view!{cx, <RoguelikeRacerGame/>}
            >
                <Lobby/>
            </Show>
        </WebsocketProvider>
    }
}
