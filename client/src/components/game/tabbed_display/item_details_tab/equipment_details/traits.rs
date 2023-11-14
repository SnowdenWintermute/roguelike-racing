use common::items::equipment::EquipmentTraits;
use yew::{html, virtual_dom::VNode};

pub fn traits(equipment_traits: &Option<Vec<EquipmentTraits>>) -> Vec<VNode> {
    let mut trait_displays = Vec::new();
    if let Some(traits) = equipment_traits {
        for equipment_trait in traits {
            let text = match equipment_trait {
                EquipmentTraits::LifeStealPercentage(value) => {
                    format!("Lifesteal: {value}%")
                }
                EquipmentTraits::DurabilityBonus(_) => format!("Increased durability"),
                EquipmentTraits::ArmorClassPercentage(value) => format!("+{value}% Armor Class"),
                EquipmentTraits::DamagePercentage(value) => format!("+{value}% Weapon damage"),
                EquipmentTraits::RandomDamageTypeSelection => {
                    format!("Damage type selected randomly")
                }
            };

            trait_displays.push(html!(
            <div>
                {text}
            </div>
            ))
        }
    }

    trait_displays
}
