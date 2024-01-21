use actix::*;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web_actors::ws;
use rand::Rng;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Instant;
use websocket_server::websocket_actor::WebsocketActor;
mod random_names;
mod utils;
mod websocket_server;

/// Entry point for our websocket route
async fn game_server_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<websocket_server::game_server::GameServer>>,
) -> Result<HttpResponse, Error> {
    let mut rng = rand::thread_rng();
    ws::start(
        WebsocketActor {
            id: rng.gen::<u32>(),
            time_of_last_ping_received: Instant::now(),
            game_server_actor_address: server.get_ref().clone(),
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
    // start chat server actor
    let game_server_actor_address = websocket_server::game_server::GameServer::new().start();

    // for i in 0..100 {
    //     let EquipmentPropertiesAndRequirements {
    //         equipment_properties,
    //         requirements,
    //     } = generate_equipment_properties_from_base_item(0);
    // }
    log::info!("starting HTTP server at http://localhost:8082");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_server_actor_address.clone()))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(game_server_route))
            .wrap(Logger::default())
    })
    .workers(1)
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
