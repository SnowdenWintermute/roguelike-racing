use common::items::equipment::EquipmentTypes;
use yew::html;
use yew::Html;

use crate::yew_app::components::game::context_dependant_information_display::damage_type_badge::DamageTypeBadge;

pub fn weapon_damage(equipment_type: &EquipmentTypes) -> Html {
    let damage = match equipment_type {
        EquipmentTypes::OneHandedMeleeWeapon(_, properties)
        | EquipmentTypes::TwoHandedMeleeWeapon(_, properties)
        | EquipmentTypes::TwoHandedRangedWeapon(_, properties) => Some(format!(
            "{}-{}",
            properties.damage.min, properties.damage.max
        )),
        _ => None,
    };

    let damage_types = match equipment_type {
        EquipmentTypes::OneHandedMeleeWeapon(_, properties)
        | EquipmentTypes::TwoHandedMeleeWeapon(_, properties)
        | EquipmentTypes::TwoHandedRangedWeapon(_, properties) => {
            Some(&properties.damage_classifications)
        }
        _ => None,
    };

    let mut classification_displays = Vec::new();

    match damage_types {
        Some(classifications) => {
            for hp_change_source in classifications {
                classification_displays
                    .push(html!( <DamageTypeBadge hp_change_source={hp_change_source.clone()} /> ))
            }
        }
        None => (),
    }

    match damage {
        Some(_) => html!(
        <div>
            <div class="mb-1">{"Damage: "}{damage}</div>
            <ul class="list-none" >
                {classification_displays}
            </ul>
        </div>
        ),
        None => html!(),
    }
}
