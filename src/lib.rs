use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use serde::Deserialize;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FormData {
    name:  String,
    email: String
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn serve(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
