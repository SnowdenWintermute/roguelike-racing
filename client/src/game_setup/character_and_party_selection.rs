use crate::{
    common_components::button_basic::ButtonBasic,
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
    let game_name = move || {
        game.get()
            .expect("if this component is showing, a game should exist")
            .name
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
            <div>"Players not yet in a party:"</div>

            <div>
                "Adventuring Parties"
                <For
                    each=move || {
                        game
                            .get()
                            .expect("should be a game if viewing game component")
                            .adventuring_parties
                            .clone()
                    }
                    key=|party| party.1.id
                    children=move |party| {
                        view! { <AdventuringPartyLobbyCard party=party.1/> }
                    }
                />
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
                        "Create Game"
                    </ButtonBasic>
                </form>
            </div>

        </section>
    }
}

#[component]
pub fn adventuring_party_lobby_card(party: AdventuringParty) -> impl IntoView {
    let characters = party.player_characters;

    view! {
        <div>
            <h3>{party.name}</h3>
            <div>
                <For
                    each=move || party.players.clone()
                    key=|player| player.1.username.clone()
                    children=move |player| {
                        let mut this_players_characters: Vec<Character> = Vec::new();
                        if let Some(ids) = &player.1.character_ids {
                            for id in ids {
                                if let Some(character) = characters.get(id) {
                                    this_players_characters.push(character.clone())
                                }
                            }
                        }
                        view! {
                            <div>
                                {player.1.username.to_string()} "characters:"
                                <For
                                    each=move || this_players_characters.clone()
                                    key=|character| character.entity_properties.id
                                    children=move |character| {
                                        view! {
                                            <div>
                                                "Class: "
                                                {match character.combatant_properties.combatant_class {
                                                    CombatantClass::Warrior => "Warrior",
                                                    CombatantClass::Rogue => "Rogue",
                                                    CombatantClass::Mage => "Mage",
                                                    _ => "Other",
                                                }}

                                            </div>
                                        }
                                    }
                                />

                            </div>
                        }
                    }
                />

            </div>
        </div>
    }
}
