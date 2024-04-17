mod website;
mod types;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Logger, NormalizePath};
use local_ip_address::local_ip;
use mongodb::{Database};
use website::website_spa;
use website::api_scope;
use crate::types::AppError;
use crate::website::utils::init_path;
use crate::website::data_repository::{init_db};

const HTTP_PORT: i32 = 8080;
const USERS_PATH_DEFAULT: &str = "/home/marius/server/bosler.it/users";

pub struct ServerState {
    users_path: PathBuf,
    database: Database,
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    println!("Starting server...");
    let user_path = init_path(env::args().nth(2), USERS_PATH_DEFAULT);

    let database : Database= init_db(user_path.clone()).await?;
    let server_data = web::Data::new(ServerState {
        users_path: user_path,
        database,
    });

    let network_ip = local_ip();
    let ip = "0.0.0.0"; // for development
    let port = HTTP_PORT;
    println!("Running on http://{ip}:{port} or http://localhost:{port} locally, which is http://{network_ip:?}:{port} on the network", ip=ip, network_ip=network_ip, port=port);
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

    //httpserver.bind(format!("{ip}:{port}", ip=ip, port=port))?.run().await
    httpserver.bind(format!("0.0.0.0:{port}", port=port))?.run().await

}


