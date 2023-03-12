use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn serve(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    //wrap a connection in a smart pointer so it can be clone
    let connection_data = web::Data::new(db_connection_pool);

    let server: Server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(connection_data.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
