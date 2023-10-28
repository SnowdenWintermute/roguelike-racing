use common::{combatants::CombatantProperties, primatives::EntityProperties};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
}

#[function_component(Combatant)]
pub fn combatant(props: &Props) -> Html {
    let Props {
        entity_properties,
        combatant_properties,
    } = props;

    // let combat_attributes = combatant_properties.clone().equipment.get_total_attributes();

    html!(
        <div class="border border-slate-400 p-2 mb-2">
            <div>
            {"entity id: "}{entity_properties.id}
            </div>
            <div class="text-green-700" >
            {"hp: "}{combatant_properties.hit_points.current}{" / "}{combatant_properties.hit_points.max}
            </div>
            <div class="text-blue-700" >
            {"hp: "}{combatant_properties.mana.current}{" / "}{combatant_properties.mana.max}
            </div>
            <div>
                <div>
                    // {"Damage: "}{combat_attributes.damage}
                </div>
                <div>
                    // {"Damage: "}{combat_attributes.armor_class}
                </div>
            </div>

        </div>
    )
}
