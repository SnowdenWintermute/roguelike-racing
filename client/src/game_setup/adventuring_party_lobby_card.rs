use std::collections::HashMap;

use crate::{
    common_components::button_basic::ButtonBasic, home_page::ClientPartyId,
    websocket_provider::send_client_input::send_client_input,
};
use common::game::getters::get_mut_player;
use common::game::RoguelikeRacerGame;
use common::packets::client_to_server::{CharacterCreation, PlayerInputs};
use common::{
    adventuring_party::AdventuringParty,
    character::{combatant_properties::CombatantClass, Character},
};
use leptos::*;
use web_sys::{SubmitEvent, WebSocket};

#[component]
pub fn adventuring_party_lobby_card(
    party: AdventuringParty,
    game: RoguelikeRacerGame,
    client_party_id: ClientPartyId,
) -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    // let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();
    let is_own_party = client_party_id.0.unwrap_or(0) == party.id;
    let characters = party.player_characters;
    let player_character_signals = party
        .player_usernames
        .iter()
        .map(|username| {
            let player_characters = game
                .get_player_characters(username.clone())
                .unwrap_or(HashMap::new());
            create_signal((username, player_characters)).0
        })
        .collect::<Vec<ReadSignal<(String, HashMap<u32, Character>)>>>();

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
                <Show when=move || is_own_party fallback=|| view! { <div></div> }>
                    <CharacterCreationMenu />
                </Show>
            </div>
            <div>
                <For
                    each=move || player_character_signals.iter()
                    key=|username_and_characters| username_and_characters.get()
                    children=move |username_and_characters| {
                        view! {
                            <div class="">
                                // <div class="mb-2">{.to_string()}</div>
                                // <div>"characters:"</div>
                                // <For
                                //     each=characters
                                //     key=|character| character
                                //     children=move |character| {
                                //         view! {
                                //         <CombatantClassDisplay character=character />
                                //         }
                                //     }
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
pub fn character_creation_menu() -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    let (character_name, set_character_name) = create_signal("".to_string());
    let disabled = MaybeSignal::derive(move || character_name().len() < 1);

    let create_character = move |e: SubmitEvent| {
        e.prevent_default();
        send_client_input(
            ws,
            PlayerInputs::CreateCharacter(CharacterCreation {
                character_name: character_name(),
                combatant_class: CombatantClass::Warrior,
            }),
        )
    };

    view! {
            <form class="flex" on:submit=create_character>
                <input
                    type="text"
                    class="bg-slate-700 border border-slate-400 h-10 p-4"
                    on:input=move |ev| {
                        set_character_name(event_target_value(&ev));
                    }

                    prop:value=character_name
                    prop:placeholder="Enter a character name..."
                />
                <ButtonBasic disabled=disabled extra_styles="border-l-0 " button_type="submit">
                    "Create Character"
                </ButtonBasic>
            </form>
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
