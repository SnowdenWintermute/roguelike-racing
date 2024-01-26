pub mod handle_cycle_targeting_schemes;
pub mod handle_cycle_targets;
pub mod handle_select_ability;
use super::enums::GameActions;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_focused_character;
use crate::store::game_store::get_item_owned_by_focused_character;
use crate::store::game_store::select_item;
use crate::store::game_store::GameStore;
use crate::store::ui_store::UIStore;
use crate::store::websocket_store::WebsocketStore;
use common::packets::client_to_server::ClientSelectAbilityPacket;
use common::packets::client_to_server::EquipItemRequest;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndItem;
use common::packets::CharacterAndSlot;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn create_action_button_click_handler<'a>(
    game_action: GameActions,
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
    ui_state: Rc<UIStore>,
    websocket_state: Rc<WebsocketStore>,
    alert_dispatch: Dispatch<AlertStore>,
) -> Box<dyn Fn()> {
    match game_action {
        GameActions::ToggleReadyToExplore => Box::new(move || {
            send_client_input(
                &websocket_state.websocket,
                PlayerInputs::ToggleReadyToExplore,
            )
        }),
        GameActions::UseAutoinjector => Box::new(move || {
            //
        }),
        GameActions::SetInventoryOpen(status) => Box::new(move || {
            game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = status.clone());
        }),
        GameActions::ToggleInventoryOpen => Box::new(move || {
            game_dispatch.reduce_mut(|game_state| {
                game_state.viewing_inventory = !game_state.viewing_inventory
            });
            game_dispatch.reduce_mut(|game_state| game_state.viewing_equipped_items = false);
        }),
        GameActions::ToggleViewingEquipedItems => Box::new(move || {
            game_dispatch.reduce_mut(|game_state| {
                game_state.viewing_equipped_items = !game_state.viewing_equipped_items
            });
        }),
        GameActions::DeselectItem => Box::new(move || {
            game_dispatch.reduce_mut(|game_state| game_state.selected_item = None);
            game_dispatch.reduce_mut(|store| {
                let parent_page_number = store.parent_menu_pages.pop();
                if let Some(page_number) = parent_page_number {
                    store.action_menu_current_page_number = page_number;
                }
                store.detailed_entity = None;
            });
        }),
        GameActions::SelectItem(id) => Box::new(move || {
            let item = get_item_owned_by_focused_character(&id, game_state.clone())
                .expect("a character should only be able to select their own items");
            let cloned_dispatch = game_dispatch.clone();
            select_item(cloned_dispatch, Some(item));
        }),
        GameActions::UseItem(_id) => Box::new(move || {
            let item_option = &game_state.selected_item;
            let character_id = game_state.focused_character_id;
            let alt_slot = ui_state.mod_key_held;
            if let Some(item) = item_option {
                game_dispatch.reduce_mut(|game_store| {
                    let focused_character =
                        get_focused_character(game_store).expect("to be in game");
                    let slot_item_is_equipped =
                        focused_character.slot_item_is_equipped(&item.entity_properties.id);
                    if let Some(slot) = slot_item_is_equipped {
                        send_client_input(
                            &websocket_state.websocket,
                            PlayerInputs::UnequipEquipmentSlot(CharacterAndSlot {
                                character_id,
                                slot,
                            }),
                        )
                    } else {
                        send_client_input(
                            &websocket_state.websocket,
                            PlayerInputs::EquipInventoryItem(EquipItemRequest {
                                character_id,
                                item_id: item.entity_properties.id,
                                alt_slot,
                            }),
                        )
                    }
                });
            }
        }),
        GameActions::SelectAbility(ability_name) => Box::new(move || {
            let cloned_dispatch = game_dispatch.clone();
            let cloned_alert_dispatch = alert_dispatch.clone();
            handle_select_ability::handle_select_ability(
                cloned_dispatch,
                cloned_alert_dispatch,
                &websocket_state.websocket,
                ability_name.clone(),
            );
        }),
        GameActions::DeselectAbility => Box::new(move || {
            game_dispatch.reduce_mut(|game_store| {
                let focused_character: _ = game_store
                    .get_mut_character(game_store.focused_character_id)
                    .expect("to have a valid focused character");
                focused_character.combatant_properties.selected_ability_name = None;
                focused_character.combatant_properties.ability_targets = None;
                send_client_input(
                    &websocket_state.websocket,
                    PlayerInputs::SelectAbility(ClientSelectAbilityPacket {
                        character_id: focused_character.entity_properties.id,
                        ability_name_option: None,
                    }),
                )
            });
        }),
        GameActions::CycleTargets(next_or_previous) => Box::new(move || {
            let cloned_dispatch = game_dispatch.clone();
            handle_cycle_targets::handle_cycle_targets(
                cloned_dispatch,
                &websocket_state.websocket,
                &next_or_previous,
            )
        }),
        GameActions::CycleTargetingScheme => Box::new(move || {
            game_dispatch.reduce_mut(|store| {
                handle_cycle_targeting_schemes::handle_cycle_targeting_schemes(
                    store,
                    &websocket_state.websocket,
                )
            })
        }),
        GameActions::UseSelectedAbility => Box::new(move || {
            game_dispatch.reduce_mut(|store| {
                let focused_character_id = store.focused_character_id;
                let focused_character = store
                    .get_mut_character(focused_character_id)
                    .expect("to have a valid focused character");
                focused_character.combatant_properties.selected_ability_name = None;
                focused_character.combatant_properties.ability_targets = None;
                send_client_input(
                    &websocket_state.websocket,
                    PlayerInputs::UseSelectedAbility(game_state.focused_character_id),
                );
            });
        }),
        GameActions::DropItem(item_id) => Box::new(move || {
            game_dispatch.reduce_mut(|store| {
                let focused_character = get_focused_character(store).expect("to be in game");
                let slot_item_is_equipped = focused_character.slot_item_is_equipped(&item_id);
                if let Some(slot) = slot_item_is_equipped {
                    send_client_input(
                        &websocket_state.websocket,
                        PlayerInputs::DropEquippedItem({
                            CharacterAndSlot {
                                character_id: store.focused_character_id,
                                slot,
                            }
                        }),
                    );
                } else {
                    send_client_input(
                        &websocket_state.websocket,
                        PlayerInputs::DropItem({
                            CharacterAndItem {
                                character_id: store.focused_character_id,
                                item_id,
                            }
                        }),
                    );
                }
                store.selected_item = None;
                store.detailed_entity = None;
            });
        }),
        GameActions::ToggleReadyToDescend => Box::new(move || {
            send_client_input(
                &websocket_state.websocket,
                PlayerInputs::ToggleReadyToGoDownStairs,
            );
        }),
        _ => Box::new(|| ()),
    }
}
