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
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!( "Adding a new subscriber.", %request_id, subscriber_email = %form.email, subscriber_name = %form.name );
    // WARN: dont use enter in async
    let _request_span_guard = request_span.enter();

    tracing::info!(
        "request_id {} - Saving new subscriber details ({}, {}) in the database",
        request_id,
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
            tracing::info!(
                "request_id {} - Subscriber details saved to database",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // note the Debug fmt specifier to capture rich information
            // which is stripped off by Display
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
    // `_request_span_guard` is dropped at the end of `subscribe` // That's when we "exit" the span
    // on the other hand when request_span itself is dropped, we "close" the span
    // We can enter and exit multiple times and closing is final
}
