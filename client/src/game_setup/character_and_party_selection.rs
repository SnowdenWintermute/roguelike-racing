use crate::{
    common_components::button_basic::ButtonBasic,
    game_setup::adventuring_party_lobby_card::AdventuringPartyLobbyCard, home_page::ClientPartyId,
    websocket_provider::send_client_input::send_client_input,
};
use common::{
    adventuring_party::AdventuringParty,
    character::{combatant_properties::CombatantClass, Character},
    game::{player_actions::PlayerInputs, RoguelikeRacerGame},
};
use leptos::{ev::SubmitEvent, *};
use web_sys::{MouseEvent, WebSocket};

#[component]
pub fn character_and_party_selection() -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();
    let party_id = expect_context::<RwSignal<ClientPartyId>>();
    let game_name = move || {
        game.get()
            .expect("if this component is showing, a game should exist")
            .name
    };
    let partyless_players = move || game.get().expect("game should exist").partyless_players;
    let adventuring_parties = move || {
        game.get()
            .expect("should be a game if viewing game component")
            .adventuring_parties
            .clone()
    };

    let (new_party_name, set_new_party_name) = create_signal("".to_string());
    let disabled = MaybeSignal::derive(move || new_party_name().len() < 1);

    // let leave_game = move |e: MouseEvent| {
    //     e.prevent_default();
    //     send_client_input(ws, PlayerInputs::LeaveGame)
    // };

    let create_party = move |e: SubmitEvent| {
        e.prevent_default();
        send_client_input(ws, PlayerInputs::CreateAdventuringParty(new_party_name()))
    };

    view! {
        <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400" id="game_list">
            <h2>"Game: " {game_name}</h2>
            <form class="flex" on:submit=create_party>
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
                <For each=partyless_players
                key=|player| player.0.clone()
                children=move |player| {
                    view! {
                        {player.0.clone()}
                    }
                }
                    />
            </div>
            <div>
                "Adventuring Parties"
                <For
                    each=adventuring_parties
                    key=|party| party.1.id
                    children=move |party| {
                        view! { <AdventuringPartyLobbyCard party=party.1 client_party_id=party_id.get() /> }
                    }
                />
            </div>
        </section>
    }
}
