mod combatant_animation_manager;
pub mod combatant_class_icon;
mod combatant_is_ally;
mod combatant_is_selected;
mod combatant_is_targeted;
mod focus_character_button;
mod process_next_action_result_in_combatant_event_queue;
mod process_next_animation_in_combatant_animation_queue;
mod value_bar;
use crate::yew_app::components::common_components::atoms::hoverable_tooltip_wrapper::HoverableTooltipWrapper;
use crate::yew_app::components::common_components::atoms::targeting_indicator::TargetingIndicator;
use crate::yew_app::components::game::combatant::combatant_animation_manager::CombatantAnimationManager;
use crate::yew_app::components::game::combatant::focus_character_button::FocusCharacterButton;
use crate::yew_app::components::game::combatant::value_bar::ValueBar;
use crate::yew_app::store::game_store::get_current_battle_option;
use crate::yew_app::store::game_store::DetailableEntities;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::game_store::{self};
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
    let cloned_game_dispatch = game_dispatch.clone();
    let handle_click = Callback::from(move |_| {
        cloned_game_dispatch.reduce_mut(|store| {
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
    let targeted_by = combatant_is_targeted::combatant_targeted_by(game_state.clone(), &id);
    let battle_option = get_current_battle_option(&game_state);
    let is_active_combatant = match battle_option {
        Some(battle) => battle.combatant_is_first_in_turn_order(id),
        None => false,
    };

    let selected_style = if is_selected { "border-yellow-400" } else { "" };

    let styles = format!(
        "flex border border-slate-400 mb-2 relative h-fit w-fit {}",
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

    let event_manager_option = game_state
        .action_results_manager
        .combantant_event_managers
        .get(&id);
    let animating = if let Some(event_manager) = event_manager_option {
        event_manager.animation_queue.len() > 0
    } else {
        false
    };

    let cloned_game_dispatch = game_dispatch.clone();
    let handle_unspent_attributes_button_click = Callback::from(move |_: MouseEvent| {
        cloned_game_dispatch.reduce_mut(|store| {
            store.focused_character_id = id;
            store.viewing_attribute_point_assignment_menu = true;
        })
    });

    let unspent_attributes_button = if is_ally && combatant_properties.unspent_attribute_points > 0
    {
        html!(
            <button onclick={handle_unspent_attributes_button_click}
                class="bg-ffxipink h-5 w-5 border border-slate-950 text-slate-950 absolute top-1 left-1 text-lg leading-3" >
                { "+" }
            </button>
        )
    } else {
        html!()
    };

    html!(
        <div class={styles} >
            {unspent_attributes_button}
            if targeted_by.len() > 0 {
                <div class="absolute top-[-1.5rem] left-1/2 -translate-x-1/2 z-20
                            flex" >
                            {targeted_by.iter().map(|combatant_id_and_with_what| html!(
                                        <TargetingIndicator
                                            combat_action={combatant_id_and_with_what.1.clone()}
                                            />
                            )).collect::<Html>()}
                    </div>
            }
            if is_active_combatant {
                <div class={format!("absolute z-50 top-1/2 -translate-y-1/2 {}
                                    pr-2 pl-2 border border-slate-400 bg-slate-700", turn_indicator_style)} >
                    {"active"}
                </div>
            }
            <button class={"flex flex-col bg-slate-700 w-40
                text-left p-2 cursor-help overflow-hidden"} onclick={handle_click} id={format!("combatant-{}", id)} >
                <div class="pointer-events-none whitespace-nowrap overflow-ellipsis" >
                    if !animating {
                        {name}
                    }
                    <CombatantAnimationManager combatant_id={id} />
                </div>
                <div class="h-5 w-full pointer-events-none" >
                {
                    if let Some(max_hp) = max_hp_option {
                        html!(<ValueBar max={max_hp} curr={combatant_properties.hit_points} color={"green-700"} />)
                    } else {
                        html!({"Immortal Object"})
                    }
                }
                </div>
                <div class="h-5 w-full pointer-events-none mb-2" >
                {
                    if let Some(max_mp) = max_mp_option {
                        html!(
                            <ValueBar max={max_mp} curr={combatant_properties.mana} color={"blue-700"} />
                            )
                    } else {
                        html!({"Infinite Mana"})
                    }
                }
                </div>
                <div class="w-full flex pointer-events-none items-end" >
                <span class="mr-2 whitespace-nowrap inline-block leading-3" >{format!( "Lv. {}", combatant_properties.level )}</span>
                {
                    if let Some(required_exp_to_level) = combatant_properties.experience_points.required_for_next_level {
                        html!(
                            <div class="h-2 w-full">
                                <ValueBar max={required_exp_to_level} curr={combatant_properties.experience_points.current} color={"ffxipink"} hide_numbers={ true } />
                            </div>
                            )
                    } else {
                        html!()
                    }
                }
                </div>
            </button>
            if is_ally {
                <FocusCharacterButton id={id} is_ally={is_ally} combatant_class={props.combatant_properties.combatant_class.clone()} />
            }
        </div>
    )
}
