use common::combat::hp_change_source_types::HpChangeSourceCategories;
use common::combat::magical_elements::MagicalElements;
use common::items::equipment::EquipmentTypes;
use yew::html;
use yew::Html;

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
                let classification_text = match classification.category {
                    HpChangeSourceCategories::PhysicalDamage => "Physical",
                    HpChangeSourceCategories::MagicalDamage(_) => "Magical",
                    HpChangeSourceCategories::Healing => "Healing",
                    HpChangeSourceCategories::Direct => "Direct",
                };
                let damage_classification_border_color = match classification.category {
                    HpChangeSourceCategories::PhysicalDamage => "border-zinc-300",
                    HpChangeSourceCategories::MagicalDamage(_) => "border-sky-300",
                    HpChangeSourceCategories::Healing => "border-green-600",
                    HpChangeSourceCategories::Direct => "border-black-300",
                };

                let damage_type_text = format!("{}", classification_text);
                let mut damage_type_color_style = "bg-zinc-300 text-slate-700";
                if let Some(element) = &classification.element {
                    damage_type_color_style = match element {
                        MagicalElements::Fire => "bg-firered",
                        MagicalElements::Ice => "bg-iceblue",
                        MagicalElements::Lightning => "bg-lightningpurple",
                        MagicalElements::Water => "bg-waterblue",
                        MagicalElements::Earth => "bg-earthyellow text-slate-700",
                        MagicalElements::Wind => "bg-windgreen text-slate-700",
                        MagicalElements::Dark => "bg-darknessblack",
                        MagicalElements::Light => "bg-lightwhite text-slate-700",
                    }
                }
                let damage_type_style = format!("pr-1 pl-1 {}", damage_type_color_style);
                classification_displays.push(html!(
                <li class={format!("border pl-1 max-w-fit mb-1 {}", damage_classification_border_color)}>
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
