use self::armor::*;
use crate::yew_app::components::game::context_dependant_information_display::item_details::equipment_details::equipment_durability::EquipmentDurability;
use crate::yew_app::components::game::context_dependant_information_display::item_details::requirements::requirements;
use crate::yew_app::components::game::context_dependant_information_display::item_details::unmet_requirements_calculator::UnmetRequirementsCalculator;
use crate::yew_app::store::game_store::GameStore;
use common::combatants::combat_attributes::CombatAttributes;
use common::items::equipment::EquipmentProperties;
use std::collections::HashMap;
use yew::prelude::*;
use yewdux::prelude::use_store;
mod armor;
mod combat_attributes_and_traits;
mod equipment_durability;
mod weapon_damage;
use self::combat_attributes_and_traits::combat_attributes_and_traits;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub equipment_properties: EquipmentProperties,
    pub requirements: Option<HashMap<CombatAttributes, u8>>,
    pub entity_id: u32,
    pub is_compared_item: bool,
}

#[function_component(EquipmentDetails)]
pub fn equipment_details(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let armor_category_if_any = armor_category(&props.equipment_properties.equipment_type);
    let armor_class_if_any = armor_class(
        &props.equipment_properties.equipment_type,
        &props.equipment_properties.traits,
    );
    let weapon_damage_if_any =
        weapon_damage::weapon_damage(&props.equipment_properties.equipment_type);

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
                {combat_attributes_and_traits(&props.equipment_properties)}
                {requirements(&props.requirements, cloned_game_state)}
                {
                    if !props.is_compared_item {
                        html!(
                            <UnmetRequirementsCalculator
                                equipment_requirements={props.requirements.clone()}
                                entity_id={props.entity_id}
                                />
                            )
                    } else {
                        html!()
                    }
                }
            </div>
    )
}
