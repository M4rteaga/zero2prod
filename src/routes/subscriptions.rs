use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    //create a identifier to correlate logs
    let request_id = Uuid::new_v4();
    //span, like logs, have associated level
    //`info_span` creates a span at the info-level
    let request_span = tracing::info_span!("Adding a new subscriber.",
        %request_id,
            subscriber_email = %form.email,
            subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    //query that inserts user
    match sqlx::query!(
        r#"
                 INSERT INTO subscriptions (id, email, name, subscribed_at)
                 VALUES ($1,$2,$3,$4)
                 "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    //first we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            tracing::error!(
                "request_id {} Failed to execute query: {:?}",
                request_id,
                err
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
