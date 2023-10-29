use crate::store::game_store::{self, DetailableEntities, GameStore};
use common::{combatants::CombatantProperties, primatives::EntityProperties};
use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
}

#[function_component(Combatant)]
pub fn combatant(props: &Props) -> Html {
    let (_, game_dispatch) = use_store::<GameStore>();
    let entity_properties = props.entity_properties.clone();
    let combatant_properties = props.combatant_properties.clone();

    let _combat_attributes = combatant_properties.clone().get_total_attributes();

    let cloned_entity_properties = entity_properties.clone();
    let cloned_combatant_properties = combatant_properties.clone();
    let handle_click = Callback::from(move |_e: MouseEvent| {
        game_dispatch.reduce_mut(|store| {
            store.detailed_entity = Some(DetailableEntities::Combatant(
                game_store::CombatantDetails {
                    entity_properties: cloned_entity_properties.clone(),
                    combatant_properties: cloned_combatant_properties.clone(),
                },
            ));
        });
    });

    let cloned = handle_click.clone();
    let click_handler_emitter = Callback::from(move |e| {
        cloned.emit(e);
    });

    let element_id = format!("combatant-{}", entity_properties.id);

    html!(
        <button class="border border-slate-400 p-2 mb-2 max-w-fit" onclick={click_handler_emitter} id={element_id}>
            <div>
            {"entity id: "}{entity_properties.id}
            </div>
            <div class="text-green-700" >
            {"hp: "}{combatant_properties.hit_points.current}{" / "}{combatant_properties.hit_points.max}
            </div>
            <div class="text-blue-700" >
            {"hp: "}{combatant_properties.mana.current}{" / "}{combatant_properties.mana.max}
            </div>
            // <div>
            //     <div>
            //         {"Damage: "}{combat_attributes.get(&CombatAttributes::Damage).unwrap_or(&0)}
            //     </div>
            //     <div>
            //         {"Armor Class: "}{combat_attributes.get(&CombatAttributes::ArmorClass).unwrap_or(&0)}
            //     </div>
            // </div>

        </button>
    )
}
