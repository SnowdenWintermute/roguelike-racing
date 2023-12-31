use super::GameServer;
use crate::websocket_server::ListRooms;
use actix::Context;
use actix::Handler;
use actix::MessageResult;

impl Handler<ListRooms> for GameServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        // let mut rooms = Vec::new();

        // for key in self.rooms.keys() {
        //     rooms.push(key.to_owned())
        // }

        MessageResult(vec!["unimplemented".to_string()])
    }
}
