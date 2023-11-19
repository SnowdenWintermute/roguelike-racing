use std::collections::HashSet;

use common::{
    combatants::{CombatAttributes, CombatantProperties},
    game::{getters::get_character, RoguelikeRacerGame},
    items::{
        equipment::{EquipableSlots, EquipmentSlots},
        Item,
    },
    primatives::EntityProperties,
};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub struct CombatantDetails {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
}

#[derive(PartialEq, Clone, Debug)]
pub enum DetailableEntities {
    Combatant(CombatantDetails),
    Item(Item),
}

impl DetailableEntities {
    pub fn get_id(&self) -> u32 {
        match self {
            DetailableEntities::Combatant(properties) => properties.entity_properties.id,
            DetailableEntities::Item(properties) => properties.entity_properties.id,
        }
    }
}

#[derive(Store, Default, PartialEq, Clone)]
pub struct GameStore {
    pub game: Option<RoguelikeRacerGame>,
    pub current_party_id: Option<u32>,
    pub detailed_entity: Option<DetailableEntities>,
    pub hovered_entity: Option<DetailableEntities>,
    pub selected_item: Option<Item>,
    pub compared_item: Option<Item>,
    pub compared_slot: Option<EquipmentSlots>,
    pub considered_item_unmet_requirements: Option<HashSet<CombatAttributes>>,
    pub focused_character_id: u32,
    pub viewing_skill_level_up_menu: bool,
    pub viewing_attribute_point_assignment_menu: bool,
    pub viewing_inventory: bool,
    pub selecting_injection_type: bool,
    pub viewing_items_on_ground: bool,
    pub parent_menu_pages: Vec<u8>,
    pub action_menu_current_page_number: u8,
}

pub fn set_item_hovered(game_dispatch: Dispatch<GameStore>, item_option: Option<Item>) {
    game_dispatch.reduce_mut(|store| {
        if let Some(item) = item_option {
            let entity_details = DetailableEntities::Item(item.clone());
            store.hovered_entity = Some(entity_details);
        } else {
            store.hovered_entity = None;
        }
    })
}

pub fn select_item(game_dispatch: Dispatch<GameStore>, item_option: Option<Item>) {
    game_dispatch.reduce_mut(|store| {
        store.selected_item = item_option.clone();
        store.hovered_entity = None;
        if let Some(item) = item_option {
            if let Some(entity) = &store.detailed_entity {
                let id = entity.get_id();
                if id == item.entity_properties.id {
                    store.detailed_entity = None
                } else {
                    store.detailed_entity = Some(DetailableEntities::Item(item));
                }
            } else {
                store.detailed_entity = Some(DetailableEntities::Item(item));
            }
        }
        store
            .parent_menu_pages
            .push(store.action_menu_current_page_number);
        store.action_menu_current_page_number = 0;
    })
}

pub fn set_compared_item<'a>(
    game_dispatch: Dispatch<GameStore>,
    item_id: u32,
    compare_alternate_slot: bool,
) {
    game_dispatch.reduce_mut(|store| {
        if let Some(game) = &mut store.game {
            if let Some(party_id) = store.current_party_id {
                if let Some(item_considering) =
                    game.get_item_in_adventuring_party(party_id, item_id)
                {
                    // get the character which we want to compare equipment
                    let focused_character = get_character(
                        game,
                        party_id,
                        store.focused_character_id,
                    )
                    .expect(
                        "we should only be focusing a character that exists in the player's party",
                    );
                    // find the equipment slot of the item
                    let slots_option: Option<EquipableSlots> =
                        match &item_considering.item_properties {
                            common::items::ItemProperties::Consumable(_) => None,
                            common::items::ItemProperties::Equipment(equipment_properties) => {
                                Some(equipment_properties.get_equippable_slots())
                            }
                        };

                    if let Some(slots) = slots_option {
                        let slot_to_compare = if let Some(alternate_slot) = slots.alternate {
                            if compare_alternate_slot {
                                alternate_slot
                            } else {
                                slots.main
                            }
                        } else {
                            slots.main
                        };
                        store.compared_slot = Some(slot_to_compare.clone());
                        let equiped_item_option = focused_character
                            .combatant_properties
                            .equipment
                            .get(&slot_to_compare);
                        match equiped_item_option {
                            Some(item) => {
                                // don't compare to self
                                if item.entity_properties.id != item_id {
                                    store.compared_item = Some(item.clone())
                                } else {
                                    store.compared_item = None
                                }
                            }
                            None => store.compared_item = None,
                        }
                    }
                }
            }
        }
    });
}
