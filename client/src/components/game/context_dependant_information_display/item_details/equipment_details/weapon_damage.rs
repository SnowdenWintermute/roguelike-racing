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
            for hp_change_source in classifications {
                let classification_text = match hp_change_source.category {
                    HpChangeSourceCategories::PhysicalDamage(_) => "Physical",
                    HpChangeSourceCategories::MagicalDamage(_) => "Magical",
                    HpChangeSourceCategories::Healing => "Healing",
                    HpChangeSourceCategories::Direct => "Direct",
                };
                let element_text_option = if let Some(element) = &hp_change_source.element {
                    Some(format!("{element}"))
                }else {
                    None
                };
                let damage_classification_border_color = match hp_change_source.category {
                    HpChangeSourceCategories::PhysicalDamage(_) => "border-zinc-300",
                    HpChangeSourceCategories::MagicalDamage(_) => "border-sky-300",
                    HpChangeSourceCategories::Healing => "border-green-600",
                    HpChangeSourceCategories::Direct => "border-black-300",
                };

                let damage_type_text_option = match &hp_change_source.sub_category {
                    Some(sub_category) => Some(format!("{}", sub_category)),
                    None => None,
                };
                let mut damage_type_color_style = "bg-zinc-300 text-slate-700";
                let element_color_style = if let Some(element) = &hp_change_source.element {
                    match element {
                        MagicalElements::Fire => "bg-firered",
                        MagicalElements::Ice => "bg-iceblue",
                        MagicalElements::Lightning => "bg-lightningpurple",
                        MagicalElements::Water => "bg-waterblue",
                        MagicalElements::Earth => "bg-earthyellow text-slate-700",
                        MagicalElements::Wind => "bg-windgreen text-slate-700",
                        MagicalElements::Dark => "bg-darknessblack",
                        MagicalElements::Light => "bg-lightwhite text-slate-700",
                    }
                } else { "" };
                let damage_type_style = format!("{}", damage_type_color_style);
                classification_displays.push(html!(
                <li class={format!("border pl-1 max-w-fit mb-1 {}", damage_classification_border_color)}>
                    <span class={format!("inline-block pr-1 h-full {}", damage_classification_border_color)}>{classification_text}{" "}</span>
                    {match damage_type_text_option {
                        Some(text) => 
                            html!(
                                <span class={format!("border-l inline-block h-full pr-1 pl-1 {}", damage_type_style)}>{text}</span>
                                )
                            ,
                        None => html!(),
                    }}

                    {match element_text_option {
                        Some(element_text) => html!(
                                <span class={format!("border-l inline-block h-full pr-1 pl-1 {}", element_color_style )}>{element_text}</span>
                            ),
                        None => html!(),
                    }}
                    
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
