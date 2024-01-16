mod combatant_animation_manager;
mod combatant_is_ally;
mod combatant_is_selected;
mod combatant_is_targeted;
pub mod create_animations_from_turn_result;
mod focus_character_button;
mod process_next_action_result_in_combatant_event_queue;
mod process_next_animation_in_combatant_animation_queue;
use crate::components::common_components::atoms::targeting_indicator::TargetingIndicator;
use crate::components::game::combatant::focus_character_button::FocusCharacterButton;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::DetailableEntities;
use crate::store::game_store::GameStore;
use crate::store::game_store::{self};
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
    let id = props.entity_properties.id;
    let name = props.entity_properties.name.clone();
    let combatant_properties = props.combatant_properties.clone();

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

    let is_ally = combatant_is_ally::combatant_is_ally(game_state.clone(), id);
    let is_selected = combatant_is_selected::combatant_is_selected(game_state.clone(), id);
    let is_targeted = combatant_is_targeted::combatant_is_targeted(game_state.clone(), &id);
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
