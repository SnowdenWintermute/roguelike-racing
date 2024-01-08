use crate::components::common_components::atoms::targeting_indicator::TargetingIndicator;
use crate::components::game::dungeon_room::focus_character_button::FocusCharacterButton;
use crate::store::game_store::get_active_combatant;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::DetailableEntities;
use crate::store::game_store::GameStore;
use crate::store::game_store::{self};
use common::combatants::abilities::AbilityTarget;
use common::combatants::abilities::FriendOrFoe;
use common::combatants::combat_attributes::CombatAttributes;
use common::combatants::CombatantProperties;
use common::primatives::EntityProperties;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
}

#[function_component(Combatant)]
pub fn combatant(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let game_option = &game_state.game;
    let party_id_option = game_state.current_party_id;
    if game_option.is_none() | party_id_option.is_none() {
        return html!({ "no game found" });
    }
    let game = game_option.as_ref().expect("is_none checked");
    let party_id = party_id_option.expect("is_none checked");
    let party_option = game.adventuring_parties.get(&party_id);
    if party_option.is_none() {
        return html!("no party found");
    }
    let party = party_option.expect("is_none checked");

    let id = props.entity_properties.id;
    let name = props.entity_properties.name.clone();
    let combatant_properties = props.combatant_properties.clone();

    let _combat_attributes = combatant_properties.clone().get_total_attributes();

    let cloned_entity_properties = props.entity_properties.clone();
    let cloned_combatant_properties = combatant_properties.clone();
    let handle_click = Callback::from(move |_| {
        game_dispatch.reduce_mut(|store| {
            store.detailed_entity = Some(DetailableEntities::Combatant(
                game_store::CombatantDetails {
                    entity_properties: cloned_entity_properties.clone(),
                    combatant_properties: cloned_combatant_properties.clone(),
                },
            ));
        });
    });

    let is_ally = {
        let mut value = false;
        if let Some(game) = &game_state.game {
            if let Some(party) = game
                .adventuring_parties
                .get(&game_state.current_party_id.expect("to be in a party"))
            {
                for (character_id, _) in party.characters.iter() {
                    if character_id == &id {
                        value = true;
                    }
                }
            }
        }
        value
    };

    let is_selected = match &game_state.detailed_entity {
        Some(combatant_details) => match combatant_details {
            DetailableEntities::Combatant(combatant_details) => {
                combatant_details.entity_properties.id == id
            }
            DetailableEntities::Item(_) => false,
        },
        None => false,
    };

    let is_targeted = {
        let active_character_result = get_active_combatant(&game_state);
        if let Ok(active_character_option) = active_character_result {
            if let Some(active_character) = active_character_option {
                let targets_option = &active_character.1.ability_targets;
                if let Some(targets) = targets_option {
                    match targets {
                        AbilityTarget::Single(targeted_id) => &id == targeted_id,
                        AbilityTarget::Group(category) => match category {
                            FriendOrFoe::Friendly => party.character_positions.contains(&id),
                            FriendOrFoe::Hostile => {
                                if let Ok(monster_ids) = party.get_monster_ids() {
                                    monster_ids.contains(&id)
                                } else {
                                    false
                                }
                            }
                        },
                        AbilityTarget::All => true,
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };

    let battle_option = get_current_battle_option(&game_state);
    let is_active_combatant = match battle_option {
        Some(battle) => battle.combatant_is_first_in_turn_order(id),
        None => false,
    };

    let selected_style = if is_selected { "border-yellow-400" } else { "" };

    let styles = format!(
        "flex border border-slate-400 mb-2 last:mb-0 w-40 relative {}",
        selected_style
    );

    let turn_indicator_style = if is_ally {
        "-right-2 translate-x-full"
    } else {
        "-left-2 -translate-x-full"
    };

    let total_attributes = combatant_properties.get_total_attributes();
    let max_hp_option = total_attributes.get(&CombatAttributes::Hp);
    let max_mp_option = total_attributes.get(&CombatAttributes::Mp);

    html!(
        <div class={styles}>
            if is_targeted{
                <div class="absolute top-[-1.5rem] left-1/2 -translate-x-1/2 z-20
                    " >
                    <TargetingIndicator />
                    </div>
            }
            if is_active_combatant {
                <div class={format!("absolute top-1/2 -translate-y-1/2 {}
                                    pr-2 pl-2 border border-slate-400 bg-slate-700", turn_indicator_style)} >
                    {"active"}
                </div>
            }
            <button class={"text-left p-2 cursor-help w-full overflow-hidden"} onclick={handle_click} id={format!("combatant-{}", id)} >
                <div class="pointer-events-none">
                    {name}
                </div>
                <div class="text-green-700 pointer-events-none" >
                {
                    if let Some(max_hp) = max_hp_option {
                        {format!("hp: {} / {}", combatant_properties.hit_points, max_hp)}
                    } else {
                        {"Immortal Object".to_string()}
                    }
                }
                </div>
                <div class="text-blue-700 pointer-events-none" >
                {
                    if let Some(max_mp) = max_mp_option {
                        {format!("mp: {} / {}", combatant_properties.mana, max_mp)}
                    } else {
                        {"Infinite Mana".to_string()}
                    }
                }
                </div>
            </button>
            if is_ally {
                <FocusCharacterButton id={id} is_ally={is_ally} />
            }
        </div>
    )
}
