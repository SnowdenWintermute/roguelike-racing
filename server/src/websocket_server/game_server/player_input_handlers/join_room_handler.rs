use common::{
    game,
    packets::server_to_client::{GameServerUpdatePackets, RoomState},
};

use crate::websocket_server::game_server::GameServer;

pub fn join_room_handler(game_server: &mut GameServer, room_name: &str, actor_id: u32) {
    let connected_user = match game_server.sessions.get_mut(&actor_id) {
        Some(user) => user,
        None => {
            println!("tried to join a room but no user was found with the provided actor_id");
            return;
        }
    };

    let username = connected_user.username.clone();
    let previous_room_name = connected_user.current_room_name.clone();
    println!("previous_room_name: {}", previous_room_name);
    connected_user.current_room_name = room_name.to_string();

    // REMOVE THEM FROM THEIR PREVIOUS ROOM
    if previous_room_name != room_name {
        let room_leaving = game_server.rooms.get_mut(&previous_room_name);
        match room_leaving {
            Some(room) => {
                room.remove(&actor_id);
                if room.len() < 1 {
                    // if empty, delete the room
                    game_server.rooms.remove(&previous_room_name);
                } else {
                    // UPDATE THEIR PREVIOUS ROOM MEMBERS
                    game_server.emit_packet(
                        &previous_room_name,
                        &GameServerUpdatePackets::UserLeftRoom(username.clone()),
                        None,
                    );
                }
            }
            None => println!("tried to remove a user from a room but no room was found"),
        }
    }

    // ADD THEM TO NEW ROOM
    game_server
        .rooms
        .entry(room_name.to_string())
        .or_default()
        .insert(actor_id);

    let new_room_usernames = game_server
        .rooms
        .get(room_name)
        .expect("this room should exist because we just created it or inserted a user into it")
        .into_iter()
        .filter_map(|id| {
            if let Some(connected_user) = game_server.sessions.get(id) {
                Some(connected_user.username.clone())
            } else {
                None
            }
        })
        .collect();

    // GIVE THEM NEW ROOM INFO
    game_server.send_packet(
        &GameServerUpdatePackets::RoomFullUpdate(RoomState {
            room_name: room_name.to_string(),
            users: new_room_usernames,
        }),
        actor_id,
    );

    // UPDATE THE NEW ROOM
    game_server.emit_packet(
        &room_name,
        &GameServerUpdatePackets::UserJoinedRoom(username),
        Some(actor_id),
    );
}
