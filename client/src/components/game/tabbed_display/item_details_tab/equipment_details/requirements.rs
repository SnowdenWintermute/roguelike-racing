use common::combatants::CombatAttributes;
use std::collections::HashMap;
use yew::{html, virtual_dom::VNode};

pub fn requirements(requirements: &HashMap<CombatAttributes, u8>) -> Vec<VNode> {
    let mut displays = Vec::new();
    for (index, (attribute, value)) in requirements.iter().enumerate() {
        if index == 0 {
            displays.push(html!(
            <div>
                {"Requirements:"}
            </div>
            ))
        }
        displays.push(html!(
        <div>
            {format!("{} {}", value, attribute)}
        </div>
        ))
    }

    displays
}
