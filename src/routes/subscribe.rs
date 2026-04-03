use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // GDPR!!!
    log::info!(
        "Saving new subscriber details ({}, {}) in the database",
        form.email,
        form.name
    );
    match sqlx::query!(
        r#" INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgPool` // wrapped by `web::Data`.
    // Using the pool as a drop-in replacement
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("Subscriber details saved to database");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // note the Debug fmt specifier to capture rich information
            // which is stripped off by Display
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
