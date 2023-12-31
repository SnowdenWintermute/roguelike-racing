use super::Character;
use crate::{
    combatants::{
        abilities::{CombatantAbility, CombatantAbilityNames},
        combat_attributes::CombatAttributes,
    },
    game::RoguelikeRacerGame,
    items::{
        equipment::equipment_generation::create_starting_equipment::create_starting_equipment, Item,
    },
};

pub fn outfit_new_warrior(game: &mut RoguelikeRacerGame, character: &mut Character) {
    let combatant_properties = &mut character.combatant_properties;
    let inherent_attributes = &mut combatant_properties.inherent_attributes;
    inherent_attributes.insert(CombatAttributes::Hp, 10);
    inherent_attributes.insert(CombatAttributes::Damage, 1);
    inherent_attributes.insert(CombatAttributes::Strength, 20);
    inherent_attributes.insert(CombatAttributes::Dexterity, 1);
    inherent_attributes.insert(CombatAttributes::Vitality, 2);
    inherent_attributes.insert(CombatAttributes::Resilience, 2);
    inherent_attributes.insert(CombatAttributes::Accuracy, 75);
    inherent_attributes.insert(CombatAttributes::Agility, 1);

    // ABILITIES
    combatant_properties.abilities.insert(
        CombatantAbilityNames::Fire,
        CombatantAbility::new(CombatantAbilityNames::Fire),
    );
    combatant_properties.abilities.insert(
        CombatantAbilityNames::Heal,
        CombatantAbility::new(CombatantAbilityNames::Heal),
    );
    combatant_properties.abilities.insert(
        CombatantAbilityNames::RainStorm,
        CombatantAbility::new(CombatantAbilityNames::RainStorm),
    );

    // TEST INVENTORY ITEMS
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

    // STARTING EQUIPMENT
    let starting_equipment = create_starting_equipment(&mut game.id_generator);
    for (slot, item) in starting_equipment {
        combatant_properties.equipment.insert(slot, item);
    }

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
