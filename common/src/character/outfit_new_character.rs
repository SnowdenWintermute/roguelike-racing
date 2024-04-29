#![allow(unused, unused_imports)]
use super::create_inventory_test_items::create_inventory_test_items;
use super::Character;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_classes::level_zero_attributes::BASE_STARTING_ATTRIBUTES;
use crate::combatants::combatant_classes::starting_traits;
use crate::combatants::combatant_classes::starting_traits::STARTING_COMBATANT_TRAITS;
use crate::combatants::combatant_classes::CombatantClass;
use crate::combatants::combatant_traits::CombatantTraits;
use crate::game::RoguelikeRacerGame;
use crate::items::consumables::ConsumableTypes;
use crate::items::equipment::equipment_generation::create_starting_equipment::create_starting_equipment;
use crate::items::Item;
use crate::items::ItemCategories;

pub fn outfit_new_character(game: &mut RoguelikeRacerGame, character: &mut Character) {
    let combatant_properties = &mut character.combatant_properties;
    combatant_properties.unspent_attribute_points = 3;

    // SET UP STARTING ATTRIBUTES
    let inherent_attributes = &mut combatant_properties.inherent_attributes;
    let starting_attributes_option =
        BASE_STARTING_ATTRIBUTES.get(&combatant_properties.combatant_class);
    if let Some(starting_attributes) = starting_attributes_option {
        for (attribute, value) in starting_attributes {
            inherent_attributes.insert(attribute.clone(), *value as u16);
        }
    }

    let mut hp = inherent_attributes
        .entry(CombatAttributes::Hp)
        .and_modify(|item| *item = 1);

    // SET UP STARTING TRAITS
    let starting_traits = match STARTING_COMBATANT_TRAITS.get(&combatant_properties.combatant_class)
    {
        Some(traits) => traits.clone(),
        None => vec![],
    };

    combatant_properties.traits = starting_traits;

    // ABILITIES
    combatant_properties.abilities.insert(
        CombatantAbilityNames::Fire,
        CombatantAbility::create_by_name(&CombatantAbilityNames::Fire),
    );
    if combatant_properties.combatant_class == CombatantClass::Mage {
        combatant_properties.abilities.insert(
            CombatantAbilityNames::Ice,
            CombatantAbility::create_by_name(&CombatantAbilityNames::Ice),
        );
    }
    combatant_properties.abilities.insert(
        CombatantAbilityNames::Healing,
        CombatantAbility::create_by_name(&CombatantAbilityNames::Healing),
    );

    let hp_injector = Item::create_consumable_by_type(
        game.id_generator.get_next_entity_id(),
        ConsumableTypes::HpAutoinjector,
    );
    let mp_injector = Item::create_consumable_by_type(
        game.id_generator.get_next_entity_id(),
        ConsumableTypes::MpAutoinjector,
    );
    combatant_properties.inventory.items.push(hp_injector);
    combatant_properties.inventory.items.push(mp_injector);

    // TEST INVENTORY ITEMS
    let mut test_items = create_inventory_test_items(game);
    combatant_properties.inventory.items.append(&mut test_items);

    // STARTING EQUIPMENT
    let starting_equipment = create_starting_equipment(
        &mut game.id_generator,
        &combatant_properties.combatant_class,
    );
    for (slot, item) in starting_equipment {
        combatant_properties.equipment.insert(slot, item);
    }

    combatant_properties.set_hp_and_mp_to_max();
}
