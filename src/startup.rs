use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn serve(listener: TcpListener, db_connection: PgConnection) -> Result<Server, std::io::Error> {
    //wrap a connection in a smart pointer so it can be clone
    let connection_data = web::Data::new(db_connection);
    let server: Server = HttpServer::new(move ||
                                         App::new()
                                         .service(health_check)
                                         .service(subscribe)
                                         .app_data(connection_data.clone())
                                         )
        .listen(listener)?
        .run();

    Ok(server)
}
