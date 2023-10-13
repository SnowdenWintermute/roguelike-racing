use crate::websocket_server::game_server::{getters::get_mut_user, GameServer};
use common::{
    app_consts::error_messages,
    errors::AppError,
    packets::server_to_client::{GameServerUpdatePackets, RoomState},
};

impl GameServer {
    pub fn join_room_handler(&mut self, room_name: &str, actor_id: u32) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let previous_room_name = connected_user.current_room_name.clone();
        println!("previous room name: {:#?}", previous_room_name);
        connected_user.current_room_name = room_name.to_string();

        // REMOVE THEM FROM THEIR PREVIOUS ROOM
        if previous_room_name != room_name {
            let room_leaving = self
                .rooms
                .get_mut(&previous_room_name)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::ROOM_NOT_FOUND.to_string(),
                })?;
            room_leaving.remove(&actor_id);
            if room_leaving.len() < 1 {
                self.rooms.remove(&previous_room_name);
            } else {
                // UPDATE THEIR PREVIOUS ROOM MEMBERS
                self.emit_packet(
                    &previous_room_name,
                    &GameServerUpdatePackets::UserLeftRoom(username.clone()),
                    None,
                )?;
            }
        }

        // ADD THEM TO NEW ROOM
        self.rooms
            .entry(room_name.to_string())
            .or_default()
            .insert(actor_id);

        let room_joined = self.rooms.get(room_name).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::ROOM_NOT_FOUND.to_string(),
        })?;

        let usernames_in_joined_room = room_joined
            .into_iter()
            .filter_map(|id| {
                if let Some(connected_user) = self.sessions.get(id) {
                    Some(connected_user.username.clone())
                } else {
                    None
                }
            })
            .collect();

        // GIVE THEM NEW ROOM INFO
        self.send_packet(
            &GameServerUpdatePackets::RoomFullUpdate(RoomState {
                room_name: room_name.to_string(),
                users: usernames_in_joined_room,
            }),
            actor_id,
        )?;

        // UPDATE THE NEW ROOM
        self.emit_packet(
            &room_name,
            &GameServerUpdatePackets::UserJoinedRoom(username),
            Some(actor_id),
        )
    }
}
