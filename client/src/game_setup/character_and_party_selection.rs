use crate::{
    common_components::button_basic::ButtonBasic,
    game_setup::adventuring_party_lobby_card::AdventuringPartyLobbyCard, home_page::ClientPartyId,
    websocket_provider::send_client_input::send_client_input,
};
use common::game::{player_actions::PlayerInputs, RoguelikeRacerGame};
use leptos::{ev::SubmitEvent, *};
use web_sys::WebSocket;

#[component]
pub fn character_and_party_selection() -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    let party_id = expect_context::<RwSignal<ClientPartyId>>();
    let game = move || match expect_context::<RwSignal<Option<RoguelikeRacerGame>>>().get() {
        Some(game) => game,
        None => RoguelikeRacerGame::new("".to_string()),
    };

    let game_name = move || game().name.clone();
    let players = move || game().players.clone();
    let adventuring_parties = move || game().adventuring_parties.clone();

    let (new_party_name, set_new_party_name) = create_signal("".to_string());
    let disabled = MaybeSignal::derive(move || new_party_name().len() < 1);

    let create_party = move |e: SubmitEvent| {
        e.prevent_default();
        send_client_input(ws, PlayerInputs::CreateAdventuringParty(new_party_name()))
    };

    view! {
        <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400" id="game_list">
            <h2>"Game: " {game_name}</h2>
            <form class="flex mb-2" on:submit=create_party>
                <input
                    type="text"
                    class="bg-slate-700 border border-slate-400 h-10 p-4"
                    on:input=move |ev| {
                        set_new_party_name(event_target_value(&ev));
                    }

                    prop:value=new_party_name
                    prop:placeholder="Enter a party name..."
                />
                <ButtonBasic disabled=disabled extra_styles="border-l-0 " button_type="submit">
                    "Create Adventuring Party"
                </ButtonBasic>
            </form>
            <div>
                <h3>"Players not yet in a party:"</h3>
                <ul class="list-none">
                    <For
                        each=players
                        key=|player| (player.1.username.clone(), player.1.party_id.is_none())
                        children=|player| player.1.party_id.is_none().then(move || 
                                view! { <li>{player.1.username.clone()}</li> }
                        )
                    />

                </ul>
            </div>
            <div>
                <h3 class="mb-2">"Adventuring Parties"</h3>
                <For
                    each=adventuring_parties
                    key=|party| party.1.id
                    children=move |party| {
                        view! {
                            <AdventuringPartyLobbyCard
                                party=party.1
                                client_party_id=party_id.get()
                            />
                        }
                    }
                />

            </div>
        </section>
    }
}
