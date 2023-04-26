mod website;
mod types;

use actix_web::{App, HttpServer};
use actix_web::middleware::{Logger, NormalizePath};
use website::website_spa;
use website::api_scope;

const HTTP_PORT: i32 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");

    let ip = website::utils::get_ip();
    let port = HTTP_PORT;
    println!("Running on http://{ip}:{port} or http://localhost:{port} locally, which is http://{network_ip}:{port} on the network", ip=ip, network_ip=ip, port=port);
    let httpserver = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::default())
            .wrap(Logger::default())
            .service(api_scope())
            .service(website_spa())
    });

    httpserver.bind(format!("{ip}:{port}", ip=ip, port=port))?.run().await
}
