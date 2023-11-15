use std::collections::HashMap;

use common::{
    combatants::{CombatAttributes, CombatantProperties},
    items::equipment::{weapon_properties::WeaponProperties, EquipmentSlots, EquipmentTraits},
};
use yew::{html, Html};

pub fn weapon_damage(
    combatant_properties: &CombatantProperties,
    total_attributes: &HashMap<CombatAttributes, u16>,
) -> Html {
    let rh_weapon_option =
        combatant_properties.get_equipped_weapon_properties(&EquipmentSlots::RightHand);
    let lh_weapon_option =
        combatant_properties.get_equipped_weapon_properties(&EquipmentSlots::LeftHand);
    let base_damage = match total_attributes.get(&CombatAttributes::Damage) {
        Some(value) => *value,
        None => 0,
    };

    html!(
        <div class="flex" >
            {weapon_damage_entry(rh_weapon_option,base_damage ,&"Right Hand", &"mr-1")}
            {weapon_damage_entry(lh_weapon_option,base_damage, &"Left Hand", &"ml-1")}
        </div>
    )
}

fn weapon_damage_entry(
    weapon_properties_option: Option<(&WeaponProperties, &Option<Vec<EquipmentTraits>>)>,
    base_damage: u16,
    label: &str,
    padding_class: &str,
) -> Html {
    if let Some((weapon_properties, _)) = weapon_properties_option {
        html!(
        <div class={format!("w-1/2 flex justify-between {}", padding_class )}>
            <span>
                {label}
            </span>
            <span>
            {
                format!("{}-{}",
                        weapon_properties.damage.min as u16 + base_damage,
                        weapon_properties.damage.max as u16 + base_damage
                        )
            }
            </span>
        </div>
        )
    } else {
        html!(<div class={format!("w-1/2 mr-1{}", padding_class)  }/>)
    }
}
