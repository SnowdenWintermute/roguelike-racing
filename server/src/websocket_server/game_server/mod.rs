use actix::prelude::*;
use common::app_consts::LOBBY_CHANNEL;
use common::app_consts::MAIN_CHAT_CHANNEL;
use common::game::RoguelikeRacerGame;
use common::packets::client_to_server::PlayerInputs;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use std::collections::HashMap;
use std::collections::HashSet;
pub mod connection_handler;
pub mod disconnection_handler;
pub mod getters;
mod join_user_to_websocket_channel;
pub mod list_rooms_handler;
pub mod player_input_handlers;
mod remove_user_from_websocket_channel;
pub mod send_messages;
pub mod update_packet_creators;
use super::AppMessage;
use super::ClientBinaryMessage;
use super::ClientMessage;
use crate::utils::generate_random_username;

#[derive(Debug)]
pub struct UserWebsocketChannels {
    pub main: (WebsocketChannelNamespace, String),
    pub party: Option<String>,
    pub chat: Option<String>,
}

#[derive(Debug)]
pub struct ConnectedUser {
    pub id: u32,
    pub websocket_actor: Recipient<AppMessage>,
    pub username: String,
    pub websocket_channels: UserWebsocketChannels,
    pub current_game_name: Option<String>,
}

impl ConnectedUser {
    pub fn new(id: u32, websocket_actor: Recipient<AppMessage>) -> Self {
        ConnectedUser {
            id,
            websocket_actor,
            username: generate_random_username(),
            websocket_channels: UserWebsocketChannels {
                main: (WebsocketChannelNamespace::Lobby, LOBBY_CHANNEL.to_string()),
                party: None,
                chat: None,
            },
            current_game_name: None,
        }
    }
}

#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<u32, ConnectedUser>,
    websocket_channels: HashMap<WebsocketChannelNamespace, HashMap<String, HashSet<u32>>>,
    games: HashMap<String, RoguelikeRacerGame>,
}

impl GameServer {
    pub fn new() -> GameServer {
        let mut websocket_channels: HashMap<
            WebsocketChannelNamespace,
            HashMap<String, HashSet<u32>>,
        > = HashMap::new();
        websocket_channels
            .entry(WebsocketChannelNamespace::Lobby)
            .or_default()
            .insert(LOBBY_CHANNEL.to_string(), HashSet::new());
        websocket_channels
            .entry(WebsocketChannelNamespace::Lobby)
            .or_default()
            .insert(MAIN_CHAT_CHANNEL.to_string(), HashSet::new());

        GameServer {
            sessions: HashMap::new(),
            websocket_channels,
            games: HashMap::new(),
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<ClientBinaryMessage> for GameServer {
    type Result = ();
    fn handle(&mut self, message: ClientBinaryMessage, _: &mut Context<Self>) {
        println!("message received: {:?}", message.content);
        let byte_slice = &message.content[..];
        let deserialized: Result<PlayerInputs, _> = serde_cbor::from_slice(byte_slice);
        println!("deserialized: {:?}", deserialized);
        let result = match deserialized {
            Ok(PlayerInputs::CreateGame(game_creation_data)) => {
                self.create_game_handler(message.actor_id, game_creation_data)
            }
            Ok(PlayerInputs::JoinGame(game_name)) => {
                self.join_game_handler(message.actor_id, game_name)
            }
            Ok(PlayerInputs::LeaveGame) => self.leave_game_handler(message.actor_id),
            Ok(PlayerInputs::RequestGameList) => {
                self.game_list_update_request_handler(message.actor_id)
            }
            Ok(PlayerInputs::CreateAdventuringParty(party_name)) => {
                self.create_adventuring_party_handler(message.actor_id, party_name)
            }
            Ok(PlayerInputs::LeaveAdventuringParty) => self.leave_party_handler(message.actor_id),
            Ok(PlayerInputs::JoinAdventuringParty(party_id)) => {
                self.join_party_handler(message.actor_id, party_id)
            }
            Ok(PlayerInputs::CreateCharacter(character_creation)) => {
                self.create_character_handler(message.actor_id, character_creation)
            }
            Ok(PlayerInputs::DeleteCharacter(id)) => {
                self.delete_character_handler(message.actor_id, id)
            }
            Ok(PlayerInputs::ToggleReady) => self.toggle_ready_handler(message.actor_id),
            Ok(PlayerInputs::EquipInventoryItem(packet)) => self.equip_item_handler(
                message.actor_id,
                packet.character_id,
                packet.item_id,
                packet.alt_slot,
            ),
            Ok(PlayerInputs::UnequipEquipmentSlot(packet)) => {
                self.unequip_slot_handler(message.actor_id, packet.character_id, packet.slot)
            }
            Ok(PlayerInputs::ToggleReadyToExplore) => {
                self.toggle_ready_to_explore_handler(message.actor_id)
            }
            Ok(PlayerInputs::ToggleReadyToGoDownStairs) => {
                self.toggle_ready_to_descend_handler(message.actor_id)
            }
            Ok(PlayerInputs::SelectCombatAction(packet)) => {
                self.character_selects_combat_action_handler(message.actor_id, packet)
            }
            Ok(PlayerInputs::UseSelectedCombatAction(packet)) => {
                self.character_uses_selected_combat_action_handler(message.actor_id, packet)
            }
            Ok(PlayerInputs::CycleCombatActionTargets(packet)) => {
                self.character_cycles_combat_action_targets_handler(message.actor_id, packet)
            }
            Ok(PlayerInputs::CycleCombatActionTargetingSchemes(packet)) => self
                .character_cycles_combat_action_targeting_schemes_handler(message.actor_id, packet),
            Ok(PlayerInputs::TakeItemOnGround(packet)) => {
                self.character_picks_up_item_from_ground_handler(message.actor_id, packet)
            }
            Ok(PlayerInputs::DropItem(packet)) => {
                self.character_drops_item(message.actor_id, packet)
            }
            Ok(PlayerInputs::DropEquippedItem(packet)) => {
                self.character_drops_equipped_item(message.actor_id, packet)
            }
            Ok(PlayerInputs::AcknowledgeReceiptOfItemOnGroundUpdate(item_id)) => self
                .client_acknowledges_receipt_of_item_on_ground_handler(message.actor_id, item_id),
            _ => {
                println! {"unhandled binary message\n {:#?}:",deserialized};
                Ok(())
            }
        };
        match result {
            Err(app_error) => {
                println!("{:#?}", app_error);
                match self.send_packet(
                    &GameServerUpdatePackets::Error(app_error.message),
                    message.actor_id,
                ) {
                    Err(app_error) => eprintln!("{:#?}", app_error),
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

impl Handler<ClientMessage> for GameServer {
    type Result = ();
    fn handle(&mut self, message: ClientMessage, _: &mut Context<Self>) {
        println!("message received: {}", message.content);
        let channel_option = &self
            .sessions
            .get(&message.actor_id)
            .expect("if we got a message from this id, the user should exist in our list")
            .websocket_channels
            .chat;

        if let Some(channel) = channel_option {
            let result = self.send_string_message(&channel, message.content.as_str());
            if result.is_err() {
                eprintln!("{:#?}", result);
            }
        }
    }
}
