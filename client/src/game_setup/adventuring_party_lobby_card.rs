use common::{
    adventuring_party::AdventuringParty,
    character::{combatant_properties::CombatantClass, Character},
};
use leptos::*;

use crate::home_page::ClientPartyId;

#[component]
pub fn adventuring_party_lobby_card(
    party: AdventuringParty,
    client_party_id: ClientPartyId,
) -> impl IntoView {
    let characters = party.player_characters;
    let is_own_party = client_party_id.0.unwrap_or(0) == party.id;

    view! {
        <div class="p-3 border border-slate-400 w-full mb-2">
            <h3 class="mb-2">"Party: "{party.name}</h3>
            <div>
            {move || if is_own_party {
                                         "This is your current party"
                             }else {
                                ""
                             }}
        </div>
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
                            <div class="">
                                <div class="mb-2">
                                    {player.1.username.to_string()}
                                </div>
                                <div>
                                    "characters:"
                                </div>
                                <For
                                    each=move || this_players_characters.clone()
                                    key=|character| character.entity_properties.id
                                    children=move |character| {
                                        view! {
                                            <CombatantClassDisplay character=character />
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
