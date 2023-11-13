use common::items::equipment::{
    weapon_properties::{DamageClassifications, DamageTypes},
    EquipmentTypes,
};
use yew::{html, Html};

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
            for classification in classifications {
                let classification_text = match classification {
                    DamageClassifications::Direct(_) => "Direct",
                    DamageClassifications::Physical(_) => "Physical",
                    DamageClassifications::Magical(_) => "Magical",
                };
                let damage_classification_border_color = match classification {
                    DamageClassifications::Direct(_) => "border-black-300",
                    DamageClassifications::Physical(_) => "border-zinc-300",
                    DamageClassifications::Magical(_) => "border-sky-300 ",
                };

                let damage_type = match classification {
                    DamageClassifications::Direct(damage_type)
                    | DamageClassifications::Physical(damage_type)
                    | DamageClassifications::Magical(damage_type) => damage_type,
                };
                let damage_type_text = format!("{}", damage_type);
                let damage_type_color_style = match damage_type {
                    DamageTypes::Pure => "bg-zinc-300 text-slate-700",
                    DamageTypes::Slashing => "bg-zinc-300 text-slate-700",
                    DamageTypes::Blunt => "bg-zinc-300 text-slate-700",
                    DamageTypes::Piercing => "bg-zinc-300 text-slate-700",
                    DamageTypes::Fire => "bg-firered",
                    DamageTypes::Ice => "bg-iceblue",
                    DamageTypes::Lightning => "bg-lightningpurple",
                    DamageTypes::Water => "bg-waterblue",
                    DamageTypes::Earth => "bg-earthyellow",
                    DamageTypes::Wind => "bg-windgreen",
                    DamageTypes::Dark => "bg-darknessblack",
                    DamageTypes::Light => "bg-lightwhite",
                };
                let damage_type_style = format!("pr-1 pl-1 {}", damage_type_color_style);
                classification_displays.push(html!(
                <li class={format!("border pl-1 max-w-fit mb-1 last:mb-0 {}", damage_classification_border_color)}>
                    <span class={format!("inline-block pr-1 border-r h-full {}", damage_classification_border_color)}>{classification_text}{" "}</span>
                    <span class={format!("inline-block h-full {}", damage_type_style)}>{damage_type_text}</span>
                </li>
                ))
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
