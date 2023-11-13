use crate::store::game_store::{self, DetailableEntities, GameStore};
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

    let selected_style = match &game_state.detailed_entity {
        Some(entity) => match entity {
            DetailableEntities::Combatant(combatant_details) => {
                if combatant_details.entity_properties.id == id {
                    "border-yellow-400"
                } else {
                    ""
                }
            }
            DetailableEntities::Item(_) => "",
        },
        None => "",
    };
    let styles = format!(
        "text-left border border-slate-400 p-2 mb-2 last:mb-0 w-40 {}",
        selected_style
    );

    html!(
        <button class={styles} onclick={handle_click} id={format!("combatant-{}", id)} >
            <div class="pointer-events-none">
            {"entity id: "}{id}
            </div>
            <div class="text-green-700 pointer-events-none" >
            {"hp: "}{combatant_properties.hit_points.current}{" / "}{combatant_properties.hit_points.max}
            </div>
            <div class="text-blue-700 pointer-events-none" >
            {"hp: "}{combatant_properties.mana.current}{" / "}{combatant_properties.mana.max}
            </div>
        </button>
    )
}
