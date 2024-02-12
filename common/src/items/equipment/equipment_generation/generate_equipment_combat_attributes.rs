use crate::app_consts::TWO_HANDED_WEAPON_ATTRIBUTE_MULTIPLIER;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combat_attributes::CORE_ATTRIBUTES;
use crate::items::equipment::affixes::Affix;
use crate::items::equipment::affixes::PrefixTypes;
use crate::items::equipment::affixes::SuffixTypes;
use crate::items::equipment::EquipmentTypes;
use rand::Rng;
use std::collections::HashMap;

struct AttributeValueCreationTemplate {
    attribute: CombatAttributes,
    min: f32,
    max: f32,
}

impl AttributeValueCreationTemplate {
    pub fn new(attribute: CombatAttributes, min: f32, max: f32) -> AttributeValueCreationTemplate {
        AttributeValueCreationTemplate {
            attribute,
            min,
            max,
        }
    }
}

pub fn generate_equipment_combat_attributes(
    affixes: &Vec<Affix>,
    equipment_type: &EquipmentTypes,
) -> HashMap<CombatAttributes, u16> {
    let mut attributes: HashMap<CombatAttributes, u16> = HashMap::new();
    let attribute_multiplier = match equipment_type {
        EquipmentTypes::TwoHandedMeleeWeapon(_, _)
        | EquipmentTypes::TwoHandedRangedWeapon(_, _) => TWO_HANDED_WEAPON_ATTRIBUTE_MULTIPLIER,
        _ => 1.0,
    };

    for affix in affixes {
        let mut attribute_templates = vec![];
        match affix {
            Affix::Prefix(prefix_type, tier) => {
                let tier = *tier as f32;
                match prefix_type {
                    PrefixTypes::Mp => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Mp,
                            tier,
                            tier * 5.0,
                        ));
                    }
                    PrefixTypes::ArmorClass => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::ArmorClass,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    PrefixTypes::Accuracy => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Accuracy,
                            tier,
                            tier * 5.0,
                        ));
                    }
                    PrefixTypes::PercentDamage => (),
                    PrefixTypes::LifeSteal => (),
                    PrefixTypes::Resilience => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Resilience,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    PrefixTypes::Evasion => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Evasion,
                            tier,
                            tier * 5.0,
                        ));
                    }
                    PrefixTypes::ArmorPenetration => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::ArmorPenetration,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    PrefixTypes::Agility => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Agility,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    PrefixTypes::Focus => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Focus,
                            tier,
                            tier * 2.0,
                        ));
                    }
                }
            }
            Affix::Suffix(suffix_type, tier) => {
                let tier = *tier as f32;
                match suffix_type {
                    SuffixTypes::Strength => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Strength,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    SuffixTypes::Intelligence => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Intelligence,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    SuffixTypes::Dexterity => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Dexterity,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    SuffixTypes::Vitality => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Vitality,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    SuffixTypes::AllBase => {
                        // calculate this one in place because making it into templates would cause
                        // each stat to be rolled seperately
                        let min = tier / 2.0;
                        let max = tier * 1.0;
                        let attribute_value =
                            rand::thread_rng().gen_range(min.round() as u16..=max.round() as u16);
                        for attribute in CORE_ATTRIBUTES {
                            // if some attribute already exists in the hashmap, add it's value to
                            // the roll before overwriting
                            let mut existing_attribute = 0;
                            if let Some(curr_value) = attributes.get(&attribute) {
                                existing_attribute += curr_value;
                            }
                            attributes
                                .insert(attribute.clone(), attribute_value + existing_attribute);
                        }
                    }
                    SuffixTypes::Hp => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Hp,
                            tier,
                            tier * 5.0,
                        ));
                    }
                    SuffixTypes::Damage => {
                        attribute_templates.push(AttributeValueCreationTemplate::new(
                            CombatAttributes::Damage,
                            tier,
                            tier * 2.0,
                        ));
                    }
                    SuffixTypes::Durability => (),
                }
            }
        }

        for template in attribute_templates {
            let attribute_value = rand::thread_rng().gen_range(
                (template.min.round() * attribute_multiplier) as u16
                    ..=(template.max.round() * attribute_multiplier) as u16,
            );
            let mut existing_attribute = 0;

            // in case we already added some stats
            if let Some(curr_value) = attributes.get(&template.attribute) {
                existing_attribute += curr_value;
            }
            attributes.insert(template.attribute, attribute_value + existing_attribute);
        }
    }

    attributes
}
