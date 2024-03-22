use crate::coinbase::ws::WsConn;
use crate::coinbase::lobby::Lobby;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let group_id = path.into_inner();

    let ws = WsConn::new(
        group_id,
        srv.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}