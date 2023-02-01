use actix_web::dev::Server;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn serve() -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run();

    Ok(server)
}
