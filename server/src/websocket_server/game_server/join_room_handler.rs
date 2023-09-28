use crate::websocket_server::Join;
use actix::prelude::*;

use super::GameServer;

impl Handler<Join> for GameServer {
    type Result = ();
    fn handle(&mut self, message: Join, _: &mut Context<Self>) {
        let Join {
            actor_id,
            room_name,
        } = message;
        let mut rooms_leaving = Vec::new();

        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&actor_id) {
                rooms_leaving.push(n.to_owned());
            }
        }

        for room in rooms_leaving {
            self.send_string_message(&room, "Someone disconnected");
        }

        self.rooms
            .entry(room_name.clone())
            .or_default()
            .insert(actor_id);

        self.send_string_message(&room_name, "Someone connected");
    }
}
