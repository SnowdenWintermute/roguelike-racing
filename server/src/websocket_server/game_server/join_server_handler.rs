use crate::websocket_server::Join;
use actix::prelude::*;

use super::GameServer;

impl Handler<Join> for GameServer {
    type Result = ();
    fn handle(&mut self, message: Join, _: &mut Context<Self>) {
        let Join {
            sender_id,
            room_name,
        } = message;
        let mut rooms_leaving = Vec::new();

        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&sender_id) {
                rooms_leaving.push(n.to_owned());
            }
        }

        for room in rooms_leaving {
            self.send_message(&room, "Someone disconnected", 0);
        }

        self.rooms
            .entry(room_name.clone())
            .or_default()
            .insert(sender_id);

        self.send_message(&room_name, "Someone connected", sender_id);
    }
}
