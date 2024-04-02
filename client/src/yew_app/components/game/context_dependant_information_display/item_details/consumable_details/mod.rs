use crate::yew_app::components::game::context_dependant_information_display::item_details::requirements::requirements;
use crate::yew_app::components::game::context_dependant_information_display::item_details::unmet_requirements_calculator::UnmetRequirementsCalculator;
use crate::yew_app::store::game_store::GameStore;
use common::combatants::combat_attributes::CombatAttributes;
use common::items::consumables::ConsumableProperties;
use std::collections::HashMap;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub consumable_properties: ConsumableProperties,
    pub requirements: Option<HashMap<CombatAttributes, u8>>,
    pub entity_id: u32,
}

#[function_component(ConsumableDetails)]
pub fn consumable_details(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();

    let cloned_game_state = game_state.clone();
    let combat_action_properties = props
        .consumable_properties
        .consumable_type
        .get_combat_action_properties();
    let mut targeting_schemes_text = String::from("");
    for (i, targeting_scheme) in combat_action_properties
        .targeting_schemes
        .iter()
        .enumerate()
    {
        targeting_schemes_text.push_str(&format!("{}", targeting_scheme));
        if i != combat_action_properties.targeting_schemes.len() - 1 {
            targeting_schemes_text.push_str(", ");
        }
    }

    html!(
            <div>
                <div>
                    {"Valid targets: "}{combat_action_properties.valid_target_categories}
                </div>
                <div>
                    {"Targeting schemes: "}{targeting_schemes_text}
                </div>
                <div>
                    {"Usable "}{format!("{}", combat_action_properties.usability_context)}
                </div>
                {requirements(&props.requirements, cloned_game_state)}
                <UnmetRequirementsCalculator
                    equipment_requirements={props.requirements.clone()}
                    entity_id={props.entity_id}
                />
            </div>
    )
}
