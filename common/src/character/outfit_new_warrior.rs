use super::Character;
use crate::{
    combatants::{
        abilities::{CombatantAbility, CombatantAbilityNames},
        CombatAttributes,
    },
    game::RoguelikeRacerGame,
    items::{
        equipment::{
            one_handed_melee_weapons::OneHandedMeleeWeapons,
            weapon_properties::{DamageClassifications, DamageTypes, WeaponProperties},
            EquipmentProperties, EquipmentSlots, EquipmentTypes,
        },
        Item, ItemProperties,
    },
    primatives::{EntityProperties, MaxAndCurrent, Range},
};
use std::collections::HashMap;

pub fn outfit_new_warrior(game: &mut RoguelikeRacerGame, character: &mut Character) {
    let combatant_properties = &mut character.combatant_properties;
    let inherent_attributes = &mut combatant_properties.inherent_attributes;
    inherent_attributes.insert(CombatAttributes::Hp, 10);
    inherent_attributes.insert(CombatAttributes::Damage, 1);
    inherent_attributes.insert(CombatAttributes::Strength, 2);
    inherent_attributes.insert(CombatAttributes::Dexterity, 1);
    inherent_attributes.insert(CombatAttributes::Vitality, 2);
    inherent_attributes.insert(CombatAttributes::Resilience, 2);
    inherent_attributes.insert(CombatAttributes::Accuracy, 75);

    combatant_properties.abilities.insert(
        CombatantAbilityNames::ArmorBreak,
        CombatantAbility::new(&CombatantAbilityNames::ArmorBreak),
    );
    combatant_properties.abilities.insert(
        CombatantAbilityNames::Heal,
        CombatantAbility::new(&CombatantAbilityNames::Heal),
    );

    let entity_properties = EntityProperties {
        id: game.id_generator.get_next_entity_id(),
        name: String::from("Pointy Stick"),
    };

    let mut equipment_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
            OneHandedMeleeWeapons::Stick,
            WeaponProperties {
                damage_classifications: vec![DamageClassifications::Physical(
                    DamageTypes::Piercing,
                )],
                damage: Range::new(1, 4),
            },
        ),
        durability: Some(MaxAndCurrent::new(15, 15)),
        attributes: HashMap::new(),
        affixes: vec![],
        requirements: HashMap::new(),
        traits: None,
    };

    equipment_properties
        .attributes
        .insert(CombatAttributes::Hp, 2);

    let starting_weapon = Item {
        entity_properties,
        item_level: 1,
        item_properties: ItemProperties::Equipment(equipment_properties),
    };
    for _ in 0..8 {
        let random_equipment = Item::generate(&mut game.id_generator, 5);
        character.inventory.items.push(random_equipment);
    }
    for _ in 0..8 {
        let random_equipment = Item::generate(&mut game.id_generator, 7);
        character.inventory.items.push(random_equipment);
    }
    for _ in 0..8 {
        let random_equipment = Item::generate(&mut game.id_generator, 8);
        character.inventory.items.push(random_equipment);
    }
    for _ in 0..8 {
        let random_equipment = Item::generate(&mut game.id_generator, 10);
        character.inventory.items.push(random_equipment);
    }

    combatant_properties
        .equipment
        .insert(EquipmentSlots::MainHand, starting_weapon.clone());

    combatant_properties
        .equipment
        .insert(EquipmentSlots::OffHand, starting_weapon);

    let total_attributes = combatant_properties.get_total_attributes();
    let max_hp_option = total_attributes.get(&CombatAttributes::Hp);
    if let Some(max_hp) = max_hp_option {
        combatant_properties.hit_points = *max_hp
    }
    let max_mana_option = total_attributes.get(&CombatAttributes::Mp);
    if let Some(max_mana) = max_mana_option {
        combatant_properties.mana = *max_mana
    }
}
