use super::adventuring_party_update_handlers::client_party_id_change_handler;
use super::adventuring_party_update_handlers::handle_adventuring_party_created;
use super::adventuring_party_update_handlers::handle_character_creation;
use super::adventuring_party_update_handlers::handle_character_deletion;
use super::adventuring_party_update_handlers::handle_player_changed_adventuring_party;
use super::battle_full_update_handler::handle_battle_full_update;
use super::character_selected_combat_action_handler::character_selected_combat_action_handler;
use super::dungeon_floor_number_changed_handler::dungeon_floor_number_changed_handler;
use super::game_full_update_handler::game_full_update_handler;
use super::handle_battle_victory_report::handle_battle_end_report;
use super::handle_character_dropped_equipped_item::handle_character_dropped_equipped_item;
use super::handle_character_dropped_item::handle_character_dropped_item;
use super::handle_character_picked_up_item::handle_character_picked_up_item;
use super::handle_combat_turn_results::handle_combat_turn_results;
use super::handle_raw_action_results::handle_raw_action_results;
use super::in_game_party_update_handlers::character_cycled_targeting_schemes_handler;
use super::in_game_party_update_handlers::character_cycled_targets_handler;
use super::in_game_party_update_handlers::character_spent_attribute_point_handler;
use super::in_game_party_update_handlers::handle_player_toggled_ready_to_descend;
use super::in_game_party_update_handlers::handle_player_toggled_ready_to_explore;
use super::in_game_party_update_handlers::new_dungeon_room_types_on_current_floor;
use super::inventory_management_update_handlers::handle_character_equipped_item;
use super::inventory_management_update_handlers::handle_character_unequipped_slot;
use super::lobby_update_handlers::handle_game_started;
use super::lobby_update_handlers::handle_player_toggled_ready;
use super::lobby_update_handlers::handle_user_joined_game;
use super::lobby_update_handlers::handle_user_left_game;
use super::new_dungeon_room_handler::handle_new_dungeon_room;
use super::new_game_message_handler::new_game_message_handler;
use super::websocket_channel_packet_handlers::handle_user_joined_websocket_channel;
use super::websocket_channel_packet_handlers::handle_user_left_websocket_channel;
use super::websocket_channel_packet_handlers::handle_websocket_channels_full_update;
use crate::yew_app::components::alerts::set_alert;
use crate::yew_app::store::alert_store::AlertStore;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::GameServerUpdatePackets;
use gloo::console::log;
use yewdux::prelude::Dispatch;

pub fn handle_packet(
    data: GameServerUpdatePackets,
    alert_dispatch: Dispatch<AlertStore>,
    lobby_dispatch: Dispatch<LobbyStore>,
    game_dispatch: Dispatch<GameStore>,
    websocket_dispatch: Dispatch<WebsocketStore>,
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
) -> Result<(), AppError> {
    // log!(format!("data from server: {:#?}", data));
    match data {
        GameServerUpdatePackets::Error(message) => {
            log!(format!("received error from server: {message}"));
            Ok(set_alert(alert_dispatch, message))
        }
        GameServerUpdatePackets::ClientUserName(username) => {
            lobby_dispatch.reduce_mut(|store| {
                store.username = username.clone();
            });
            Ok(())
        }
        GameServerUpdatePackets::FullUpdate(update) => {
            lobby_dispatch.reduce_mut(|store| {
                store.game_list = update.game_list.games;
            });
            websocket_dispatch
                .reduce_mut(|store| store.websocket_channels = update.websocket_channels);
            Ok(())
        }
        GameServerUpdatePackets::WebsocketChannelFullUpdate(update) => Ok(
            handle_websocket_channels_full_update(websocket_dispatch, update),
        ),
        GameServerUpdatePackets::UserLeftWebsocketChannel(packet) => Ok(
            handle_user_left_websocket_channel(websocket_dispatch, packet),
        ),
        GameServerUpdatePackets::UserJoinedWebsocketChannel(packet) => Ok(
            handle_user_joined_websocket_channel(websocket_dispatch, packet),
        ),
        GameServerUpdatePackets::GameList(update) => {
            Ok(lobby_dispatch.reduce_mut(|store| store.game_list = update.games))
        }
        GameServerUpdatePackets::GameFullUpdate(update) => {
            game_full_update_handler(game_dispatch, update)
        }
        GameServerUpdatePackets::UserJoinedGame(username) => {
            game_dispatch.reduce_mut(|store| handle_user_joined_game(store, username))
        }
        GameServerUpdatePackets::UserLeftGame(username) => {
            game_dispatch.reduce_mut(|store| handle_user_left_game(store, username))
        }
        GameServerUpdatePackets::AdventuringPartyCreated(party_creation) => game_dispatch
            .reduce_mut(|store| handle_adventuring_party_created(store, party_creation)),
        GameServerUpdatePackets::ClientAdventuringPartyId(update) => {
            client_party_id_change_handler(game_dispatch, websocket_dispatch, update)
        }
        GameServerUpdatePackets::PlayerChangedAdventuringParty(update) => {
            game_dispatch.reduce_mut(|store| handle_player_changed_adventuring_party(store, update))
        }
        GameServerUpdatePackets::CharacterCreation(character_in_party) => {
            game_dispatch.reduce_mut(|store| handle_character_creation(store, character_in_party))
        }
        GameServerUpdatePackets::CharacterDeletion(character_deletion) => {
            game_dispatch.reduce_mut(|store| handle_character_deletion(store, character_deletion))
        }
        GameServerUpdatePackets::PlayerToggledReady(username) => {
            game_dispatch.reduce_mut(|store| handle_player_toggled_ready(store, username))
        }
        GameServerUpdatePackets::GameStarted(timestamp) => {
            game_dispatch.reduce_mut(move |store| handle_game_started(store, timestamp))
        }
        GameServerUpdatePackets::CharacterEquippedItem(packet) => {
            let username = lobby_dispatch.reduce_mut(|store| store.username.clone());
            handle_character_equipped_item(
                game_dispatch,
                bevy_communication_dispatch,
                packet,
                &username,
            )
        }
        GameServerUpdatePackets::CharacterUnequippedSlot(packet) => {
            handle_character_unequipped_slot(game_dispatch, bevy_communication_dispatch, packet)
        }
        GameServerUpdatePackets::PlayerToggledReadyToExplore(username) => {
            handle_player_toggled_ready_to_explore(game_dispatch, username)
        }
        GameServerUpdatePackets::DungeonRoomUpdate(new_room) => {
            handle_new_dungeon_room(game_dispatch, bevy_communication_dispatch, new_room)
        }
        GameServerUpdatePackets::CharacterSelectedCombatAction(packet) => {
            character_selected_combat_action_handler(game_dispatch, packet)
        }
        GameServerUpdatePackets::CharacterCycledCombatActionTargets(packet) => {
            character_cycled_targets_handler(game_dispatch, packet)
        }
        GameServerUpdatePackets::CharacterCycledCombatActionTargetingSchemes(character_id) => {
            character_cycled_targeting_schemes_handler(game_dispatch, character_id)
        }
        GameServerUpdatePackets::ActionResults(packet) => {
            handle_raw_action_results(bevy_communication_dispatch, packet)
        }
        GameServerUpdatePackets::CombatTurnResults(packet) => {
            handle_combat_turn_results(bevy_communication_dispatch, packet.turn_results)
        }
        GameServerUpdatePackets::BattleFullUpdate(packet) => {
            handle_battle_full_update(game_dispatch, packet)
        }
        GameServerUpdatePackets::BattleEndReport(packet) => {
            handle_battle_end_report(game_dispatch, websocket_dispatch, packet)
        }
        GameServerUpdatePackets::CharacterPickedUpItem(packet) => {
            handle_character_picked_up_item(game_dispatch, bevy_communication_dispatch, packet)
        }
        GameServerUpdatePackets::CharacterDroppedItem(packet) => handle_character_dropped_item(
            game_dispatch,
            websocket_dispatch,
            bevy_communication_dispatch,
            packet,
        ),
        GameServerUpdatePackets::CharacterDroppedEquippedItem(packet) => {
            handle_character_dropped_equipped_item(
                game_dispatch,
                websocket_dispatch,
                bevy_communication_dispatch,
                packet,
            )
        }
        GameServerUpdatePackets::PlayerToggledReadyToDescend(packet) => {
            handle_player_toggled_ready_to_descend(game_dispatch, packet)
        }
        GameServerUpdatePackets::DungeonFloorNumber(packet) => {
            dungeon_floor_number_changed_handler(game_dispatch, packet)
        }
        GameServerUpdatePackets::DungeonRoomTypesOnCurrentFloor(packet) => {
            new_dungeon_room_types_on_current_floor(game_dispatch, packet)
        }
        GameServerUpdatePackets::GameMessage(packet) => {
            new_game_message_handler(game_dispatch, packet)
        }
        GameServerUpdatePackets::CharacterSpentAttributePoint(character_id, attribute) => {
            character_spent_attribute_point_handler(game_dispatch, character_id, &attribute)
        }
        _ => {
            log!(format!("unhandled packet: {:#?}", data));
            Err(AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::UNRECOGNIZED_PACKET.to_string(),
            })
        }
    }
}
