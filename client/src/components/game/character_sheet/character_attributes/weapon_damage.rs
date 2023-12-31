use common::{
    combatants::{CombatAttributes, CombatantProperties},
    items::equipment::EquipmentSlots,
};
use std::collections::HashMap;
use yew::{html, Html};

pub fn weapon_damage(
    combatant_properties: &CombatantProperties,
    total_attributes: &HashMap<CombatAttributes, u16>,
) -> Html {
    // @TODO - fix this to match with attack ability handler numbers
    //
    let mh_weapon_option =
        combatant_properties.get_equipped_weapon_properties(&EquipmentSlots::MainHand);
    let oh_weapon_option =
        combatant_properties.get_equipped_weapon_properties(&EquipmentSlots::OffHand);

    let base_damage = match total_attributes.get(&CombatAttributes::Damage) {
        Some(value) => *value,
        None => 0,
    };

    let accuracy = total_attributes
        .get(&CombatAttributes::Accuracy)
        .unwrap_or_else(|| &0);

    let mh_damage_and_acc_option = if let Some(mh_weapon) = mh_weapon_option {
        Some(CombatantProperties::get_weapon_damage_and_hit_chance(
            &mh_weapon.0,
            &mh_weapon.1,
            base_damage,
            *accuracy,
            false,
        ))
    } else {
        None
    };

    let modified_oh_damage_and_acc = if let Some(oh_weapon) = oh_weapon_option {
        Some(CombatantProperties::get_weapon_damage_and_hit_chance(
            &oh_weapon.0,
            &oh_weapon.1,
            base_damage,
            *accuracy,
            true,
        ))
    } else {
        None
    };

    html!(
        <div class="flex" >
            {weapon_damage_entry(mh_damage_and_acc_option, &"Main Hand", &"mr-1")}
            {weapon_damage_entry(modified_oh_damage_and_acc, &"Off Hand", &"ml-1")}
        </div>
    )
}

fn weapon_damage_entry(
    damage_and_accuracy_option: Option<(common::primatives::Range<u16>, u16)>,
    label: &str,
    padding_class: &str,
) -> Html {
    if let Some(damage_and_accuracy) = damage_and_accuracy_option {
        let damage = damage_and_accuracy.0;
        let accuracy = damage_and_accuracy.1;

        html!(
        <div class={format!("w-1/2 {}", padding_class )}>
            <div class="w-full flex justify-between">
                <span>
                    {label}
                </span>
                <span>
                    {format!("{}-{}",damage.min,damage.max)}
                </span>
            </div>
            <div class="w-full flex justify-between">
                <span>
                    {"Accuracy"}
                </span>
                <span>
                    {accuracy}
                </span>
            </div>
        </div>
        )
    } else {
        html!(<div class={format!("w-1/2 mr-1{}", padding_class)  }/>)
    }
}
