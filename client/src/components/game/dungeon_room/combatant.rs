use crate::{
    components::common_components::atoms::targeting_indicator::TargetingIndicator,
    store::game_store::{self, DetailableEntities, GameStore},
};
use common::{combatants::CombatantProperties, primatives::EntityProperties};

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

    let is_selected = match &game_state.detailed_entity {
        Some(combatant_details) => match combatant_details {
            DetailableEntities::Combatant(combatant_details) => {
                combatant_details.entity_properties.id == id
            }
            DetailableEntities::Item(_) => false,
        },
        None => false,
    };
    let selected_style = if is_selected { "border-yellow-400" } else { "" };

    let styles = format!(
        "text-left border border-slate-400 p-2 mb-2 last:mb-0 w-40 relative {}",
        selected_style
    );

    let total_attributes = combatant_properties.get_total_attributes();
    let max_hp_option = total_attributes.get(&common::combatants::CombatAttributes::Hp);
    let max_mp_option = total_attributes.get(&common::combatants::CombatAttributes::Mp);

    html!(
        <button class={styles} onclick={handle_click} id={format!("combatant-{}", id)} >
            if is_selected{
                <div class="absolute top-[-1.5rem] left-1/2 -translate-x-1/2
                    " >
                    <TargetingIndicator />
                </div>
            }
            <div class="pointer-events-none">
            {"entity id: "}{id}
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
    )
}
