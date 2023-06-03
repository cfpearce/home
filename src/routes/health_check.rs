use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    let request_id = Uuid::new_v4();
    tracing::info!("{} - Health Check", request_id);
    HttpResponse::Ok().finish()
}
