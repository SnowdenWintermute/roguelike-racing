use common::{
    adventuring_party::AdventuringParty,
    character::{combatant_properties::CombatantClass, Character},
    game::{getters::get_mut_player, player_actions::PlayerInputs, RoguelikeRacerGame},
};
use leptos::*;
use web_sys::WebSocket;

use crate::{
    common_components::button_basic::ButtonBasic, home_page::ClientPartyId,
    websocket_provider::send_client_input::send_client_input,
};

#[component]
pub fn adventuring_party_lobby_card(
    party: AdventuringParty,
    client_party_id: ClientPartyId,
) -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    // let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();
    // let characters = party.player_characters;
    let is_own_party = client_party_id.0.unwrap_or(0) == party.id;

    let leave_party = move |_| send_client_input(ws, PlayerInputs::LeaveAdventuringParty);

    view! {
        <div class="p-3 border border-slate-400 w-full mb-2">
            <h3 class="mb-2">"Party: " {party.name}</h3>
            <div>
                <Show when=move || is_own_party fallback=|| view! { <div></div> }>
                    <ButtonBasic on:click=leave_party>"Leave Party"</ButtonBasic>
                </Show>
            </div>
            <div>
                <For
                    each=move || party.player_usernames.clone()
                    key=|username| username.clone()
                    children=move |username| {
                        view! {
                            // let game_option = move || game.get();
                            // // match game_option
                            // let player = get_mut_player(, );
                            // let mut this_players_characters: Vec<Character> = Vec::new();
                            // if let Some(ids) = &player.1.character_ids {
                            // for id in ids {
                            // if let Some(character) = characters.get(id) {
                            // this_players_characters.push(character.clone())
                            // }
                            // }
                            // }
                            <div class="">
                                <div class="mb-2">{username.to_string()}</div>
                                <div>"characters:"</div>
                            // <For
                            // each=move || this_players_characters.clone()
                            // key=|character| character.entity_properties.id
                            // children=move |character| {
                            // view! {
                            // <CombatantClassDisplay character=character />
                            // }
                            // }
                            // />
                            </div>
                        }
                    }
                />

            </div>
        </div>
    }
}

#[component]
pub fn combatant_class_display(character: Character) -> impl IntoView {
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
