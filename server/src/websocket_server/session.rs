use crate::websocket_server;
use actix::prelude::*;
// use actix_web::web::Buf;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsChatSession {
    pub id: usize,
    pub time_of_last_ping_received: Instant,
    pub game_server_actor_address: Addr<websocket_server::game_server::GameServer>,
}

impl WsChatSession {
    fn heartbeat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |act, context| {
            if Instant::now().duration_since(act.time_of_last_ping_received) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.game_server_actor_address
                    .do_send(websocket_server::Disconnect { actor_id: act.id });
                context.stop();
                return;
            }

            context.ping(b"");
        });
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let actor_address = context.address();
        self.game_server_actor_address
            .send(websocket_server::Connect {
                actor_id: self.id,
                actor_address: actor_address.recipient(),
            })
            .into_actor(self)
            .then(|response, actor, context| {
                match response {
                    Ok(response) => actor.id = response,
                    _ => context.stop(), // something is wrong with chat server
                }
                fut::ready(())
            })
            .wait(context);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.game_server_actor_address
            .do_send(websocket_server::Disconnect { actor_id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to client websocket
impl Handler<websocket_server::AppMessage> for WsChatSession {
    type Result = ();
    fn handle(&mut self, message: websocket_server::AppMessage, context: &mut Self::Context) {
        match message.0 {
            websocket_server::MessageContent::Str(string_message) => context.text(string_message),
            websocket_server::MessageContent::Bytes(byte_message) => context.binary(byte_message),
        }
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self,
        message: Result<ws::Message, ws::ProtocolError>,
        context: &mut Self::Context,
    ) {
        let message = match message {
            Err(_) => {
                context.stop();
                return;
            }
            Ok(message) => message,
        };

        match message {
            ws::Message::Ping(message) => {
                self.time_of_last_ping_received = Instant::now();
                context.pong(&message);
            }
            ws::Message::Pong(_) => self.time_of_last_ping_received = Instant::now(),
            ws::Message::Binary(bytes) => {
                self.game_server_actor_address
                    .do_send(websocket_server::ClientBinaryMessage {
                        actor_id: self.id,
                        content: bytes.clone().to_vec(),
                    })
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                println!("{}", text);
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => {
                            // Send ListRooms message to chat server and wait for
                            // response
                            println!("List rooms");
                            self.game_server_actor_address
                                .send(websocket_server::ListRooms)
                                .into_actor(self)
                                .then(|response, _, context| {
                                    match response {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                context.text(room);
                                            }
                                        }
                                        _ => println!("Something is wrong"),
                                    }
                                    fut::ready(())
                                })
                                .wait(context)
                            // .wait(ctx) pauses all events in context,
                            // so actor wont receive any new messages until it get list
                            // of rooms back
                        }
                        "/join" => {
                            if v.len() == 2 {
                                self.game_server_actor_address
                                    .do_send(websocket_server::Join {
                                        actor_id: self.id,
                                        room_name: v[1].to_owned(),
                                    });

                                context.text("joined");
                            } else {
                                context.text("!!! room name is required");
                            }
                        }
                        _ => context.text(format!("!!! unknown command: {m:?}")),
                    }
                } else {
                    let message_content = m.to_owned();
                    // send message to chat server
                    self.game_server_actor_address
                        .do_send(websocket_server::ClientMessage {
                            actor_id: self.id,
                            content: message_content,
                        })
                }
            }
            ws::Message::Close(reason) => {
                context.close(reason);
                context.stop();
            }
            ws::Message::Continuation(_) => {
                context.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
