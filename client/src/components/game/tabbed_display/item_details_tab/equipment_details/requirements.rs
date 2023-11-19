use common::combatants::CombatAttributes;
use std::collections::HashMap;
use yew::{html, virtual_dom::VNode};

const UNMET_REQUIREMENT_TEXT_COLOR: &str = "text-red-400";

pub fn requirements(
    requirements: &HashMap<CombatAttributes, u8>,
    combatant_attributes: &HashMap<CombatAttributes, u16>,
) -> Vec<VNode> {
    let mut displays = Vec::new();
    for (index, (attribute, requirement_value)) in requirements.iter().enumerate() {
        if index == 0 {
            displays.push(html!(
            <div>
                {"Requirements:"}
            </div>
            ))
        }

        let character_attribute_option = combatant_attributes.get(attribute);
        let unmet_requirement_class = match character_attribute_option {
            Some(attr_value) => {
                if *attr_value >= *requirement_value as u16 {
                    ""
                } else {
                    UNMET_REQUIREMENT_TEXT_COLOR
                }
            }
            None => UNMET_REQUIREMENT_TEXT_COLOR,
        };
        displays.push(html!(
        <div class={format!("{}", unmet_requirement_class)}>
            {format!("{} {}", requirement_value, attribute)}
        </div>
        ))
    }

    displays
}
