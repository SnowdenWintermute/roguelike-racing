use crate::yew_app::components::client_consts::UNMET_REQUIREMENT_TEXT_COLOR;
use crate::yew_app::store::game_store::GameStore;
use common::combatants::combat_attributes::CombatAttributes;
use std::collections::HashMap;
use std::rc::Rc;
use yew::html;
use yew::virtual_dom::VNode;

pub fn requirements(
    requirements_option: &Option<HashMap<CombatAttributes, u8>>,
    game_state: Rc<GameStore>,
) -> Vec<VNode> {
    let mut displays = Vec::new();
    if let Some(requirements) = requirements_option {
        for (index, (attribute, requirement_value)) in requirements.iter().enumerate() {
            if index == 0 {
                displays.push(html!(
                <div>
                    {"Requirements:"}
                </div>
                ))
            }

            let requirement_is_unmet = match &game_state.considered_item_unmet_requirements {
                Some(attributes) => attributes.get(attribute).is_some(),
                None => false,
            };

            let unmet_requirement_class = if requirement_is_unmet {
                UNMET_REQUIREMENT_TEXT_COLOR
            } else {
                ""
            };

            displays.push(html!(
            <div class={format!("{}", unmet_requirement_class)}>
                {format!("{} {}", requirement_value, attribute)}
            </div>
            ))
        }
    }

    displays
}
