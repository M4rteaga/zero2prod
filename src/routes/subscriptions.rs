use serde::Deserialize;
use actix_web::{post, Responder, HttpResponse, web};

#[derive(Deserialize)]
struct FormData {
    name:  String,
    email: String
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
