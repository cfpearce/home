use actix_web::{http::header::ContentType, web, HttpResponse};
use sqlx::SqlitePool;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Subscription {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[tracing::instrument(name = "Getting subscribers")]
pub async fn get_subscriptions(pool: web::Data<SqlitePool>) -> HttpResponse {
    let res = sqlx::query_as!(
        Subscription,
        r#"
            select
                id,
                email,
                name
            from
                subscriptions
        "#
    )
    .fetch_all(pool.as_ref())
    .await;
    match res {
        Err(e) => {
            tracing::error!("Failed to get subscriptions: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(subscribers) => {
            let content = serde_json::to_string(&subscribers).unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(content)
        }
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &SqlitePool,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            insert into subscriptions(email, name)
            values( $1, $2 )
        "#,
        form.email,
        form.name
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
