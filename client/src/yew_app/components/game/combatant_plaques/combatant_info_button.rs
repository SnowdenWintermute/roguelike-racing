use crate::yew_app::store::game_store::CombatantDetails;
use crate::yew_app::store::game_store::DetailableEntities;
use crate::yew_app::store::game_store::GameStore;
use common::packets::CharacterId;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant_id: CharacterId,
    pub info_button_is_hovered: UseStateHandle<bool>,
}

#[function_component(CombatantInfoButton)]
pub fn combatant_info_button(props: &Props) -> Html {
    let combatant_id = props.combatant_id;
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let game = game_state.game.as_ref().expect("to be in a game");
    let (entity_properties, combatant_properties) = game
        .get_combatant_by_id(&props.combatant_id)
        .expect("to have a reference to a valid combatant");

    let cloned_entity_properties = entity_properties.clone();
    let cloned_combatant_properties = combatant_properties.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let handle_info_click = Callback::from(move |_| {
        cloned_game_dispatch.reduce_mut(|store| {
            let should_set_entity_detailed = match &store.detailed_entity {
                Some(detailable) => match &detailable {
                    DetailableEntities::Combatant(combatant_details) => {
                        if combatant_details.entity_properties.id == combatant_id {
                            false
                        } else {
                            true
                        }
                    }
                    DetailableEntities::Item(_) => true,
                },
                None => true,
            };

            if should_set_entity_detailed {
                store.detailed_entity = Some(DetailableEntities::Combatant(CombatantDetails {
                    entity_properties: cloned_entity_properties.clone(),
                    combatant_properties: cloned_combatant_properties.clone(),
                }))
            } else {
                store.detailed_entity = None
            }
        });
    });

    let cloned_entity_properties = entity_properties.clone();
    let cloned_combatant_properties = combatant_properties.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_info_button_is_hovered = props.info_button_is_hovered.clone();
    let handle_mouse_enter = Callback::from(move |_| {
        cloned_game_dispatch.reduce_mut(|store| {
            store.hovered_entity = Some(DetailableEntities::Combatant(CombatantDetails {
                entity_properties: cloned_entity_properties.clone(),
                combatant_properties: cloned_combatant_properties.clone(),
            }))
        });
        cloned_info_button_is_hovered.set(true)
    });

    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_info_button_is_hovered = props.info_button_is_hovered.clone();
    let handle_mouse_leave = Callback::from(move |_| {
        cloned_game_dispatch.reduce_mut(|store| store.hovered_entity = None);
        cloned_info_button_is_hovered.set(false)
    });

    html!(
    <button
        onclick={handle_info_click}
        onmouseenter={handle_mouse_enter}
        onmouseleave={handle_mouse_leave}
        class="hover:bg-slate-950 hover:border-slate-400 rounded-full leading-4">
        {"ⓘ "}
    </button>
    )
}
