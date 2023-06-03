use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let query_span = tracing::info_span!("Saving a new subscriber details in the database");

    let query = sqlx::query!(
        r#"
            insert into subscriptions(email, name)
            values( $1, $2 )
        "#,
        form.email,
        form.name
    )
    .execute(pool.get_ref())
    .instrument(query_span);

    match query.await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
