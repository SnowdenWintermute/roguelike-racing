use crate::components::mesh_manager::ActionResultsManager;
use common::adventuring_party::AdventuringParty;
use common::app_consts::error_messages::{self};
use common::character::Character;
use common::combat::battle::Battle;
use common::combatants::combat_attributes::CombatAttributes;
use common::combatants::CombatantProperties;
use common::errors::AppError;
use common::game::getters::get_character;
use common::game::RoguelikeRacerGame;
use common::items::equipment::EquipableSlots;
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use common::packets::server_to_client::BattleEndReportPacket;
use common::primatives::EntityProperties;
use std::collections::HashSet;
use std::rc::Rc;
use yew::AttrValue;
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
    pub action_results_manager: ActionResultsManager,
    pub current_party_id: Option<u32>,
    pub current_battle_id: Option<u32>,
    pub current_battle_end_report: Option<BattleEndReportPacket>,
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
    pub viewing_equipped_items: bool,
    pub selecting_injection_type: bool,
    pub viewing_items_on_ground: bool,
    pub parent_menu_pages: Vec<u8>,
    pub action_menu_current_page_number: u8,
    pub combat_log: Vec<AttrValue>,
}

pub fn get_current_party_option<'a>(game_state: &'a GameStore) -> Option<&'a AdventuringParty> {
    let game_option = &game_state.game;
    match game_option {
        Some(game) => match game_state.current_party_id {
            Some(party_id) => match game.adventuring_parties.get(&party_id) {
                Some(party) => Some(party),
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

pub fn get_current_battle_option<'a>(game_state: &'a GameStore) -> Option<&'a Battle> {
    let game_option = &game_state.game;
    match game_option {
        Some(game) => match game_state.current_party_id {
            Some(party_id) => match game.adventuring_parties.get(&party_id) {
                Some(party) => match party.battle_id {
                    Some(battle_id) => match game.battles.get(&battle_id) {
                        Some(battle) => Some(battle),
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

pub fn _get_current_battle_option_mut<'a>(game_state: &'a mut GameStore) -> Option<&'a mut Battle> {
    let game_option = &mut game_state.game;
    match game_option {
        Some(game) => match game_state.current_party_id {
            Some(party_id) => match game.adventuring_parties.get_mut(&party_id) {
                Some(party) => match party.battle_id {
                    Some(battle_id) => match game.battles.get_mut(&battle_id) {
                        Some(battle) => Some(battle),
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

pub fn get_cloned_current_battle_option(game_state: &GameStore) -> Option<Battle> {
    let game_option = &game_state.game;
    match game_option {
        Some(game) => match game_state.current_party_id {
            Some(party_id) => match game.adventuring_parties.get(&party_id) {
                Some(party) => match party.battle_id {
                    Some(battle_id) => match game.battles.get(&battle_id) {
                        Some(battle) => Some(battle.clone()),
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

pub fn get_focused_character<'a>(game_state: &'a GameStore) -> Result<&'a Character, AppError> {
    let game = game_state.game.as_ref().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::MISSING_GAME_REFERENCE.to_string(),
    })?;
    let party_id = game_state.current_party_id.ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
    })?;
    let focused_character = get_character(&game, party_id, game_state.focused_character_id);
    focused_character
}

pub fn get_active_combatant<'a>(
    game_state: &'a GameStore,
) -> Result<Option<(&'a EntityProperties, &'a CombatantProperties)>, AppError> {
    if let Some(cloned_battle) = get_current_battle_option(game_state).clone() {
        let active_combatant_turn_tracker = cloned_battle
            .combatant_turn_trackers
            .first()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::Generic,
                message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
            })?;
        let (ally_battle_group, _) = cloned_battle
            .get_ally_and_enemy_battle_groups(&active_combatant_turn_tracker.entity_id)?;
        let game = game_state.game.as_ref().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })?;

        let combatant_party = game
            .adventuring_parties
            .get(&ally_battle_group.party_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            })?;

        let (entity_properties, combatant_properties) =
            combatant_party.get_combatant_by_id(&active_combatant_turn_tracker.entity_id)?;

        Ok(Some((entity_properties, combatant_properties)))
    } else {
        Ok(None)
    }
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
                    store.detailed_entity = None;
                    store.selected_item = None
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
                        let equipped_item_option = focused_character
                            .combatant_properties
                            .equipment
                            .get(&slot_to_compare);
                        match equipped_item_option {
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

pub fn get_item_owned_by_focused_character(
    id: &u32,
    game_state: Rc<GameStore>,
) -> Result<Item, AppError> {
    let character = get_focused_character(&game_state)?;

    for (_, item) in &character.combatant_properties.equipment {
        if item.entity_properties.id == *id {
            return Ok(item.clone());
        }
    }

    for item in &character.inventory.items {
        if item.entity_properties.id == *id {
            return Ok(item.clone());
        }
    }

    return Err(AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::INVALID_ITEM_ID.to_string(),
    });
}
