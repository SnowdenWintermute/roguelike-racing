use actix::*;
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use websocket_server::websocket_actor::WebsocketActor;
mod websocket_server;

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<websocket_server::game_server::GameServer>>,
) -> Result<HttpResponse, Error> {
    let mut rng = rand::thread_rng();
    ws::start(
        WebsocketActor {
            id: rng.gen::<usize>(),
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
    // set up applications state
    let visitor_count = Arc::new(AtomicUsize::new(0));
    // start chat server actor
    let game_server_actor_address =
        websocket_server::game_server::GameServer::new(visitor_count.clone()).start();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(visitor_count.clone()))
            .app_data(web::Data::new(game_server_actor_address.clone()))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
            .wrap(Logger::default())
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
