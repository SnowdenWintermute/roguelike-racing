use common::{
    combatants::{CombatAttributes, CORE_ATTRIBUTES},
    items::equipment::{affixes::Affix, EquipmentProperties},
};
use gloo::console::log;
use yew::{html, virtual_dom::VNode};

pub fn combat_attributes(equipment_properties: &EquipmentProperties) -> Vec<VNode> {
    let mut displayed_attributes = Vec::new();

    let mut attributes_as_vec = equipment_properties
        .attributes
        .clone()
        .into_iter()
        .collect::<Vec<(CombatAttributes, u16)>>();
    attributes_as_vec.sort_by(move |a, b| a.0.partial_cmp(&b.0).unwrap());

    // DISPLAY CORE CORE_ATTRIBUTES AS ONE LINE
    // find out if it has all core attributes affix
    if has_base_stats_affix(&equipment_properties.affixes) {
        let mut cloned_attributes = equipment_properties.attributes.clone();
        let cloned_attributes_as_vec = cloned_attributes
            .clone()
            .into_iter()
            .collect::<Vec<(CombatAttributes, u16)>>();
        let mut core_attribute_values = Vec::new();
        for (attribute, value) in &cloned_attributes_as_vec {
            for core_attribute in CORE_ATTRIBUTES {
                if attribute == &core_attribute {
                    core_attribute_values.push((attribute, value))
                }
            }
        }
        // calculate the lowest number of dex int str vit resil
        let mut lowest_core_attribute_value = None;
        for (_attribute, value) in &core_attribute_values {
            if lowest_core_attribute_value.is_none() {
                lowest_core_attribute_value = Some(value)
            } else if lowest_core_attribute_value.expect("is_none checked") > value {
                lowest_core_attribute_value = Some(value)
            }
        }
        let lowest_core_attribute_value =
            **lowest_core_attribute_value.expect("should be calculated above");

        for (attribute, value) in core_attribute_values {
            // if any number higher than the lowest, subtract the lowest from it
            if value > &lowest_core_attribute_value {
                let value_to_modify = cloned_attributes.get(&attribute).expect("");
                let new_value = value_to_modify - lowest_core_attribute_value;
                log!(format!(
                    "subtracting {} from {} got {}",
                    lowest_core_attribute_value, value_to_modify, new_value
                ));
                cloned_attributes.insert(*attribute, new_value);
            }
            // remove all base stats that tied for lowest
            else {
                cloned_attributes.remove(&attribute);
            }
        }
        let mut cloned_attributes_as_vec = cloned_attributes
            .clone()
            .into_iter()
            .collect::<Vec<(CombatAttributes, u16)>>();
        cloned_attributes_as_vec.sort_by(move |a, b| a.0.partial_cmp(&b.0).unwrap());
        for (attribute, value) in cloned_attributes_as_vec {
            displayed_attributes.push(html!(
            <div>
            {format!("{}: {}", attribute, value)}
            </div>
            ))
        }
        displayed_attributes.push(html!(
        <div>
        {format!("{}: {}", "All Core Attributes", lowest_core_attribute_value)}
        </div>
        ))
        // push "all base" and the lowest
    } else {
        attributes_as_vec = equipment_properties
            .attributes
            .clone()
            .into_iter()
            .collect::<Vec<(CombatAttributes, u16)>>();
        attributes_as_vec.sort_by(move |a, b| a.0.partial_cmp(&b.0).unwrap());
        for (attribute, value) in attributes_as_vec {
            displayed_attributes.push(html!(
            <div>
            {format!("{}: {}", attribute, value)}
            </div>
            ))
        }
    }
    //

    displayed_attributes
}

fn has_base_stats_affix(affixes: &Vec<Affix>) -> bool {
    for affix in affixes {
        match affix {
            Affix::Suffix(affix_type, _) => match affix_type {
                common::items::equipment::affixes::SuffixTypes::AllBase => return true,
                _ => (),
            },
            _ => (),
        }
    }
    false
}
