use actix_web::{HttpRequest, Responder, HttpResponse, HttpServer, App, web};
use actix_web::dev::Server;
use std::net::TcpListener;

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
