use common::combat::hp_change_source_types::HpChangeSource;
use common::combat::hp_change_source_types::HpChangeSourceCategories;
use yew::prelude::*;

use crate::utils::get_magical_element_tailwind_color;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hp_change_source: HpChangeSource,
}

#[function_component(DamageTypeBadge)]
pub fn damage_type_badge(props: &Props) -> Html {
    let damage_category_text = format!("{}", &props.hp_change_source.category);

    let damage_category_border_color = match &props.hp_change_source.category {
        HpChangeSourceCategories::PhysicalDamage(_) => "border-zinc-300",
        HpChangeSourceCategories::MagicalDamage(_) => "border-sky-300",
        HpChangeSourceCategories::Healing => "border-green-600",
        HpChangeSourceCategories::Direct => "border-black-300",
    };

    let damage_category_bg_and_text_color = match &props.hp_change_source.category {
        _ => "", // HpChangeSourceCategories::PhysicalDamage(_) => "",
                 // HpChangeSourceCategories::MagicalDamage(_) => "bg-sky-300 text-slate-700",
                 // HpChangeSourceCategories::Healing => "bg-green-600",
                 // HpChangeSourceCategories::Direct => "",
    };

    let physical_damage_type_text_option = match &props.hp_change_source.sub_category {
        Some(sub_category) => Some(format!("{}", sub_category)),
        None => None,
    };

    let physical_damage_type_style = format!(
        "bg-zinc-300 text-slate-700 {}",
        damage_category_border_color,
    );

    let element_text_option = if let Some(element) = &props.hp_change_source.element {
        Some(format!("{element}"))
    } else {
        None
    };

    let element_color_style = if let Some(element) = &props.hp_change_source.element {
        get_magical_element_tailwind_color(&element)
    } else {
        "".to_string()
    };

    html!(
        <div class={format!("border-2 max-w-fit mb-1 {}", damage_category_border_color)}>
            <span class={format!("inline-block pl-1 pr-1 h-full {}", damage_category_bg_and_text_color)} >{damage_category_text}</span>
            {if let Some(physical_damage_type_text) = physical_damage_type_text_option {
                html!(
                    <span class={format!("border-l-2 inline-block h-full pr-1 pl-1 {}", physical_damage_type_style)}>{physical_damage_type_text}</span>
                    )
            } else {html!()}}

            {match element_text_option {
                Some(element_text) => html!(
                        <span class={format!("border-l-2 inline-block h-full pr-1 pl-1 {} {}",
                        element_color_style, damage_category_border_color )}>{element_text}</span>
                    ),
                None => html!(),
            }}

        </div>
    )
}
