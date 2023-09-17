use crate::lobby::Lobby;
use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{fut, ActorContext};
use actix::{Actor, ActorFuture, Addr, ContextFutureSpawner, Running, StreamHandler, WrapFuture};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    room_id: Uuid,
    lobby_address: Addr<Lobby>,
    heartbeat: Instant,
    id: Uuid,
}

impl WsConn {
    pub fn new(room_id: Uuid, lobby: Addr<Lobby>) -> WsConn {
        WsConn {
            id: Uuid::new_v4(),
            room_id,
            heartbeat: Instant::now(),
            lobby_address: lobby,
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);
        let address = context.address();
        self.lobby_address
            .send(Connect {
                address: address.recipient(),
                lobby_id: self.room_id,
                sender_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, context| {
                match res {
                    Ok(_res) => (),
                    _ => context.stop(),
                }
                fut::ready(())
            })
            .wait()
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_address.do_send(Disconnect {
            sender_id: self.id,
            room_id: self.room_id,
        });
        Running::Stop
    }
}

impl WsConn {
    fn heartbeat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                println!("client timed out, disconnecting");
                actor.lobby_address.do_send(Disconnect {
                    sender_id: actor.id,
                    room_id: actor.room_id,
                });
                context.stop();
                return;
            }

            context.ping(b"PING");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(
        &mut self,
        message: Result<ws::Message, ws::ProtocolError>,
        context: &mut Self::Context,
    ) {
        match message {
            Ok(ws::Message::Ping(message)) => {
                self.heartbeat = Instant::now();
                context.pong(&message);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => context.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                context.close(reason);
                context.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                context.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => self.lobby_address.do_send(ClientActorMessage {
                sender_id: self.id,
                content: s.to_string(),
                room_id: self.room_id,
            }),
            Err(e) => {
                println!("Error when receiving websocket message: {}", e);
                context.stop();
            }
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, message: WsMessage, context: &mut Self::Context) {
        context.text(message.0);
    }
}
