use common::combatants::combat_attributes::CombatAttributes;
use common::combatants::combat_attributes::CORE_ATTRIBUTES;
use common::items::equipment::affixes::Affix;
use common::items::equipment::affixes::PrefixTypes;
use common::items::equipment::affixes::SuffixTypes;
use common::items::equipment::EquipmentProperties;
use common::items::equipment::EquipmentTraits;
use gloo::console::log;
use std::collections::VecDeque;
use yew::html;
use yew::virtual_dom::VNode;
use yew::Html;

pub fn combat_attributes_and_traits(equipment_properties: &EquipmentProperties) -> Vec<VNode> {
    let mut equipment_mods_displays_in_prefix_suffix_order = VecDeque::new();
    let lowest_core_attribute_value_option = get_lowest_core_attribute_value(equipment_properties);
    let has_base_stats_affix = has_base_stats_affix(&equipment_properties.affixes);

    for affix in &equipment_properties.affixes {
        let text = match affix {
            Affix::Prefix(prefix_type, _) => match prefix_type {
                PrefixTypes::Mp => format_bonus(equipment_properties, &CombatAttributes::Mp),
                PrefixTypes::Accuracy => {
                    format_bonus(equipment_properties, &CombatAttributes::Accuracy)
                }
                PrefixTypes::PercentDamage | PrefixTypes::LifeSteal => {
                    log!(format!(
                        "calling format trait for prefix type :{:?}",
                        prefix_type
                    ));
                    format_trait(equipment_properties, affix).expect("to have this trait")
                }
                PrefixTypes::ArmorClass => "".to_string(),
                PrefixTypes::Resilience => {
                    format_bonus(equipment_properties, &CombatAttributes::Resilience)
                }
                PrefixTypes::Evasion => {
                    format_bonus(equipment_properties, &CombatAttributes::Evasion)
                }
                PrefixTypes::Obscurity => {
                    format_bonus(equipment_properties, &CombatAttributes::Obscurity)
                }
                PrefixTypes::ArmorPenetration => {
                    format_bonus(equipment_properties, &CombatAttributes::ArmorPenetration)
                }
                PrefixTypes::Agility => {
                    format_bonus(equipment_properties, &CombatAttributes::Agility)
                }
            },
            Affix::Suffix(_, _) => "".to_string(),
        };
        if text != "".to_string() {
            equipment_mods_displays_in_prefix_suffix_order.push_back(text);
        }
    }
    for affix in &equipment_properties.affixes {
        let text = match affix {
            Affix::Prefix(_, _) => "".to_string(),
            Affix::Suffix(suffix_type, _) => match suffix_type {
                SuffixTypes::Strength => format_core_attribute_bonus(
                    equipment_properties,
                    &CombatAttributes::Strength,
                    has_base_stats_affix,
                    lowest_core_attribute_value_option,
                ),
                SuffixTypes::Intelligence => format_core_attribute_bonus(
                    equipment_properties,
                    &CombatAttributes::Intelligence,
                    has_base_stats_affix,
                    lowest_core_attribute_value_option,
                ),
                SuffixTypes::Dexterity => format_core_attribute_bonus(
                    equipment_properties,
                    &CombatAttributes::Dexterity,
                    has_base_stats_affix,
                    lowest_core_attribute_value_option,
                ),
                SuffixTypes::Vitality => format_core_attribute_bonus(
                    equipment_properties,
                    &CombatAttributes::Vitality,
                    has_base_stats_affix,
                    lowest_core_attribute_value_option,
                ),
                SuffixTypes::AllBase => format!(
                    "+{} to all core attributes",
                    lowest_core_attribute_value_option.expect("to have core attribute bonuses")
                ),
                SuffixTypes::Hp => format_bonus(equipment_properties, &CombatAttributes::Hp),
                SuffixTypes::Focus => format_bonus(equipment_properties, &CombatAttributes::Focus),
                SuffixTypes::Damage => {
                    format_bonus(equipment_properties, &CombatAttributes::Damage)
                }
                SuffixTypes::Durability => {
                    format_trait(equipment_properties, affix).expect("to have this trait")
                }
            },
        };
        if text != "".to_string() {
            equipment_mods_displays_in_prefix_suffix_order.push_back(text);
        }
    }

    let mut displays: Vec<Html> = Vec::new();
    while equipment_mods_displays_in_prefix_suffix_order.len() > 0 {
        let display_text = equipment_mods_displays_in_prefix_suffix_order
            .pop_front()
            .expect("length checked above");
        displays.push(html!(
        <div>
            {display_text}
        </div>
        ))
    }

    displays
}

fn format_bonus(
    equipment_properties: &EquipmentProperties,
    attribute: &CombatAttributes,
) -> String {
    let bonus = equipment_properties
        .attributes
        .get(attribute)
        .expect("to have the attribute");
    format!("+{bonus} {}", attribute)
}

fn format_core_attribute_bonus(
    equipment_properties: &EquipmentProperties,
    attribute: &CombatAttributes,
    has_base_stats_affix: bool,
    lowest_core_attribute_value_option: Option<u16>,
) -> String {
    let total_value = equipment_properties
        .attributes
        .get(attribute)
        .expect("to have the attribute");
    let value_not_including_all_base = if has_base_stats_affix {
        if let Some(lowest_core_attribute_value) = lowest_core_attribute_value_option {
            *total_value - lowest_core_attribute_value
        } else {
            *total_value
        }
    } else {
        *total_value
    };

    format!("+{value_not_including_all_base} {}", attribute)
}

fn get_lowest_core_attribute_value(equipment_properties: &EquipmentProperties) -> Option<u16> {
    let mut core_attribute_values = Vec::new();
    for (attribute, value) in &equipment_properties.attributes {
        for core_attribute in CORE_ATTRIBUTES {
            if attribute == &core_attribute {
                core_attribute_values.push((attribute, value))
            }
        }
    }
    let mut lowest_core_attribute_value_option = None;
    for (_, value) in &core_attribute_values {
        if let Some(lowest_value) = lowest_core_attribute_value_option {
            if lowest_value > **value {
                lowest_core_attribute_value_option = Some(**value)
            }
        } else {
            lowest_core_attribute_value_option = Some(**value)
        }
    }
    lowest_core_attribute_value_option
}

fn format_trait(equipment_properties: &EquipmentProperties, affix: &Affix) -> Option<String> {
    for equipment_trait in equipment_properties
        .traits
        .as_ref()
        .expect("to have traits")
    {
        match &affix {
            Affix::Prefix(affix_type, _) => match affix_type {
                PrefixTypes::PercentDamage => match equipment_trait {
                    EquipmentTraits::DamagePercentage(_) => {
                        return Some(format!("{}", equipment_trait))
                    }
                    _ => (),
                },
                PrefixTypes::LifeSteal => match equipment_trait {
                    EquipmentTraits::LifeStealPercentage(_) => {
                        return Some(format!("{}", equipment_trait))
                    }
                    _ => (),
                },
                PrefixTypes::ArmorClass => match equipment_trait {
                    EquipmentTraits::ArmorClassPercentage(_) => {
                        return Some(format!("{}", equipment_trait))
                    }
                    _ => (),
                },
                _ => (),
            },
            Affix::Suffix(affix_type, _) => match affix_type {
                SuffixTypes::Durability => match equipment_trait {
                    EquipmentTraits::DurabilityBonus(_) => {
                        return Some(format!("{}", equipment_trait))
                    }
                    _ => (),
                },
                _ => (),
            },
        }
    }
    None
}

fn has_base_stats_affix(affixes: &Vec<Affix>) -> bool {
    for affix in affixes {
        match affix {
            Affix::Suffix(affix_type, _) => match affix_type {
                SuffixTypes::AllBase => return true,
                _ => (),
            },
            _ => (),
        }
    }
    false
}
