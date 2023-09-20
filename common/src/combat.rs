use crate::{
    character::{Character, CharacterAbility},
    items::{Item, ItemCategories, ItemProperties},
};

pub enum CombatAction {
    UseCharacterAbility(CharacterAbility),
    UseItem(Item),
}

impl Character {
    pub fn perform_combat_action(&self, combat_action: CombatAction) {
        if self.current_room.monster.is_none() {
            println!("can't perform a combat action without an opponent");
            return;
        }

        match combat_action {
            CombatAction::UseItem(item) => match item.item_properties {
                ItemProperties::Consumable(properties) => {
                    if properties.uses_remaining < 1 {
                        println!("can't use an item with no uses remaining");
                        return;
                    }
                }
                ItemProperties::Equipment(_properties) => {
                    println!("can't use an equipment item, must use consumable");
                    return;
                }
            },
            CombatAction::UseCharacterAbility(ability) => {
                if ability.mana_cost as u16 > self.mana.current {
                    println!("not enough mana to use ability");
                    return;
                }
            }
        }
    }
}
