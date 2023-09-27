use actix::*;
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use common::game::RoguelikeRacerGame;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use websocket_server::session::WsChatSession;
mod websocket_server;

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<websocket_server::game_server::GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            time_of_last_ping_received: Instant::now(),
            current_game_id: None,
            current_room: websocket_server::MAIN_CHAT_ROOM.to_owned(),
            username: None,
            server_address: server.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // set up applications state
    let visitor_count = Arc::new(AtomicUsize::new(0));
    let games: HashMap<String, RoguelikeRacerGame> = HashMap::new();
    // start chat server actor
    let server = websocket_server::game_server::GameServer::new(visitor_count.clone()).start();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(visitor_count.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
            .wrap(Logger::default())
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn main() -> io::Result<()> {
//     let mut game = game::Game::new();
//     let mut id_generator = game::IdGenerator {
//         last_assigned_entity_id: 0,
//     };
//     let mike_email = "mikey@mikesilverman.net";
//     game.add_player_character(
//         &mut id_generator,
//         mike_email,
//         common::character::combatant_properties::CombatantClass::Mage,
//     );

//     Ok(())
// }
