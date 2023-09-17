use crate::lobby::Lobby;
use crate::ws::WsConn;
use actix::Addr;
use actix_web::{
    get, web::Data, web::Path, web::Payload, Error as ActixError, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path(group_id): Path<Uuid>,
    server: Data<Addr<Lobby>>,
) -> Result<HttpResponse, ActixError> {
    let ws = WsConn::new(group_id, server.get_ref().clone());
    let response = ws::start(ws, &req, stream)?;
    Ok(response)
}

