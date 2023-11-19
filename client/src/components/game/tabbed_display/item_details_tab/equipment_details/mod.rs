use crate::{
    components::game::tabbed_display::item_details_tab::equipment_details::equipment_durability::EquipmentDurability,
    store::game_store::GameStore,
};
use common::{
    game::getters::get_character,
    items::equipment::{EquipmentProperties, EquipmentTypes},
};
use gloo::console::log;
use std::{collections::HashSet, ops::Deref};
use yew::prelude::*;
use yewdux::prelude::use_store;
mod combat_attributes;
mod equipment_durability;
mod requirements;
mod traits;
mod weapon_damage;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub equipment_properties: EquipmentProperties,
    pub entity_id: u32,
    pub is_compared_item: bool,
}

#[function_component(EquipmentDetails)]
pub fn equipment_details(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let armor_category_if_any = armor_category(&props.equipment_properties.equipment_type);
    let armor_class_if_any = armor_class(&props.equipment_properties.equipment_type);
    let weapon_damage_if_any =
        weapon_damage::weapon_damage(&props.equipment_properties.equipment_type);

    let game = &game_state.deref().game.as_ref().expect("");
    let current_party_id = game_state.clone().current_party_id.expect("");
    let focused_character_id = game_state.clone().focused_character_id;
    let focused_character = get_character(*game, current_party_id, focused_character_id);
    let focused_character_combat_attributes = &focused_character
        .expect("")
        .combatant_properties
        .get_total_attributes();

    // SET UNMET REQUIREMENT FLAGS
    let cloned_focused_character_combat_attributes = focused_character_combat_attributes.clone();
    let cloned_equipment_requirements = props.equipment_properties.requirements.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let entity_id = props.entity_id;
    let is_compared_item = props.is_compared_item;
    use_effect_with(
        (
            cloned_focused_character_combat_attributes,
            entity_id,
            is_compared_item,
        ),
        move |(character_attributes, _, _)| {
            let mut unmet_requirement_attributes = HashSet::new();
            for (attribute, value) in &cloned_equipment_requirements {
                if is_compared_item {
                    break;
                }
                let character_attribute_option = character_attributes.get(attribute);
                match character_attribute_option {
                    Some(attr_value) => {
                        if *attr_value >= *value as u16 {
                            continue;
                        } else {
                            unmet_requirement_attributes.insert(attribute.clone())
                        }
                    }
                    None => unmet_requirement_attributes.insert(attribute.clone()),
                };
            }
            if unmet_requirement_attributes.len() > 0 {
                cloned_game_dispatch.reduce_mut(|store| {
                    store.considered_item_unmet_requirements = Some(unmet_requirement_attributes)
                })
            } else {
                cloned_game_dispatch
                    .reduce_mut(|store| store.considered_item_unmet_requirements = None)
            };

            move || {
                cloned_game_dispatch
                    .reduce_mut(|store| store.considered_item_unmet_requirements = None);
            }
        },
    );

    let cloned_game_state = game_state.clone();
    html!(
            <div>
                <div class="" >
                    {format!("{}", props.equipment_properties.equipment_type)}
                    {armor_category_if_any}
                </div>
                {armor_class_if_any}
                {weapon_damage_if_any}
                <EquipmentDurability
                    durability_option={props.equipment_properties.durability.clone()}
                    equipment_type={props.equipment_properties.equipment_type.clone()}
                />
                {combat_attributes::combat_attributes(&props.equipment_properties)}
                {traits::traits(&props.equipment_properties.traits)}
                {requirements::requirements(&props.equipment_properties.requirements, cloned_game_state)}

            </div>
    )
}

fn armor_category(equipment_type: &EquipmentTypes) -> String {
    match equipment_type {
        EquipmentTypes::BodyArmor(_, properties) | EquipmentTypes::HeadGear(_, properties) => {
            format!(" ({})", properties.armor_category)
        }
        _ => String::from(""),
    }
}
fn armor_class(equipment_type: &EquipmentTypes) -> Html {
    let armor_class_option = match equipment_type {
        EquipmentTypes::BodyArmor(_, properties) | EquipmentTypes::HeadGear(_, properties) => {
            Some(format!("Armor Class: {}", properties.armor_class))
        }
        EquipmentTypes::Shield(_, properties) => {
            Some(format!("Armor Class: {}", properties.armor_class))
        }
        _ => None,
    };
    if let Some(ac) = armor_class_option {
        html!(
        <div>
            {ac}
        </div>
        )
    } else {
        html!()
    }
}
