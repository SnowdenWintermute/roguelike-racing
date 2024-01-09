use super::adventuring_party_update_handlers::handle_adventuring_party_created;
use super::adventuring_party_update_handlers::handle_character_creation;
use super::adventuring_party_update_handlers::handle_character_deletion;
use super::adventuring_party_update_handlers::handle_player_changed_adventuring_party;
use super::in_game_party_update_handlers::handle_battle_full_update;
use super::in_game_party_update_handlers::handle_character_ability_selection;
use super::in_game_party_update_handlers::handle_character_changed_targets;
use super::in_game_party_update_handlers::handle_new_dungeon_room;
use super::in_game_party_update_handlers::handle_player_toggled_ready_to_explore;
use super::inventory_management_update_handlers::handle_character_equipped_item;
use super::inventory_management_update_handlers::handle_character_unequipped_slot;
use super::lobby_update_handlers::handle_game_started;
use super::lobby_update_handlers::handle_player_toggled_ready;
use super::lobby_update_handlers::handle_user_joined_game;
use super::lobby_update_handlers::handle_user_left_game;
use super::websocket_channel_packet_handlers::handle_user_joined_websocket_channel;
use super::websocket_channel_packet_handlers::handle_user_left_websocket_channel;
use super::websocket_channel_packet_handlers::handle_websocket_channels_full_update;
use crate::components::alerts::set_alert;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::websocket_store::WebsocketStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::GameServerUpdatePackets;
use gloo::console::log;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn handle_packet(
    data: GameServerUpdatePackets,
    alert_dispatch: Dispatch<AlertStore>,
    lobby_dispatch: Dispatch<LobbyStore>,
    lobby_state: Rc<LobbyStore>,
    game_dispatch: Dispatch<GameStore>,
    websocket_dispatch: Dispatch<WebsocketStore>,
) -> Result<(), AppError> {
    match data {
        GameServerUpdatePackets::Error(message) => Ok(set_alert(alert_dispatch, message)),
        GameServerUpdatePackets::ClientUserName(username) => {
            Ok(lobby_dispatch.reduce_mut(|store| {
                store.username = username;
            }))
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
        GameServerUpdatePackets::GameFullUpdate(update) => game_dispatch.reduce_mut(|store| {
            store.game = update;
            Ok(())
        }),
        GameServerUpdatePackets::UserJoinedGame(username) => {
            game_dispatch.reduce_mut(|store| handle_user_joined_game(store, username))
        }
        GameServerUpdatePackets::UserLeftGame(username) => {
            game_dispatch.reduce_mut(|store| handle_user_left_game(store, username))
        }
        GameServerUpdatePackets::AdventuringPartyCreated(party_creation) => game_dispatch
            .reduce_mut(|store| handle_adventuring_party_created(store, party_creation)),
        GameServerUpdatePackets::ClientAdventuringPartyId(update) => {
            if update.is_none() {
                websocket_dispatch.reduce_mut(|store| store.websocket_channels.party = None)
            }
            Ok(game_dispatch.reduce_mut(|store| {
                store.current_party_id = update;
            }))
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
            game_dispatch.reduce_mut(|store| {
                handle_character_equipped_item(store, packet, &lobby_state.username)
            })
        }
        GameServerUpdatePackets::CharacterUnequippedSlot(packet) => {
            game_dispatch.reduce_mut(|store| handle_character_unequipped_slot(store, packet))
        }
        GameServerUpdatePackets::PlayerToggledReadyToExplore(username) => game_dispatch
            .reduce_mut(|store| handle_player_toggled_ready_to_explore(store, username)),
        GameServerUpdatePackets::DungeonRoomUpdate(new_room) => {
            game_dispatch.reduce_mut(|store| handle_new_dungeon_room(store, new_room))
        }
        GameServerUpdatePackets::CharacterSelectedAbility(packet) => {
            game_dispatch.reduce_mut(|store| handle_character_ability_selection(store, packet))
        }
        GameServerUpdatePackets::CharacterChangedTargets(packet) => {
            game_dispatch.reduce_mut(|store| handle_character_changed_targets(store, packet))
        }
        GameServerUpdatePackets::ActionResults(packet) => todo!(),
        GameServerUpdatePackets::CombatTurnResults(packet) => todo!(),
        GameServerUpdatePackets::BattleFullUpdate(packet) => {
            game_dispatch.reduce_mut(|store| handle_battle_full_update(store, packet))
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
