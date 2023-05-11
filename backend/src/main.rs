mod website;
mod types;

use std::env;
use std::path::PathBuf;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Logger, NormalizePath};
use mongodb::{Database};
use website::website_spa;
use website::api_scope;
use crate::website::utils::init_path;
use crate::website::data_repository::{init_db};

const HTTP_PORT: i32 = 8080;
const USERS_PATH_DEFAULT: &str = "/home/marius/server/bosler.it/users";
pub struct ServerState {
    users_path: PathBuf,
    database: mongodb::Database
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    let user_path = init_path(env::args().nth(2), USERS_PATH_DEFAULT);

    let database : Database= init_db(user_path.clone()).await.unwrap_or_else(|err| {
        panic!("\nFailed to connect to database: \n-----------------\n{}\n-----------------\nIs the mongodb setup and the service started?\n", err);
    });
    let server_data = web::Data::new(ServerState {
        users_path: user_path,
        database
    });

    let ip = website::utils::get_ip();
    let port = HTTP_PORT;
    println!("Running on http://{ip}:{port} or http://localhost:{port} locally, which is http://{network_ip}:{port} on the network", ip=ip, network_ip=ip, port=port);
    // print database
    println!("Database: {}", server_data.database.name());
    let httpserver = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::default())
            .wrap(Logger::default())
            .app_data(server_data.clone())
            .service(api_scope())
            .service(website_spa())
    });

    httpserver.bind(format!("{ip}:{port}", ip=ip, port=port))?.run().await
}


