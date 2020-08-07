use actix_web::{HttpRequest, Responder};
use tracing::info;

pub async fn greet(req: HttpRequest) -> impl Responder {
    info!("Say greet!");
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
