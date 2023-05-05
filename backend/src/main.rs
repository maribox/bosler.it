mod website;
mod types;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Logger, NormalizePath};
use chrono::Utc;
use futures::{StreamExt, TryFutureExt};
use mongodb::{bson, Client, Collection, Database};
use mongodb::bson::{Bson, doc, Document, to_bson};
use mongodb::options::ClientOptions;
use website::website_spa;
use website::api_scope;
use crate::types::{FileInfoRec, Visibility};
use crate::website::utils::init_path;
use crate::website::data_repository::read_dir_recursive;

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

async fn init_db(users_path: PathBuf) -> Result<Database, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("bosler.it".to_string());
    let client = Client::with_options(client_options)?;
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    let database = client.database("Website");
    let userfiles = database.collection::<Document>("Userfiles");
    match find_top_level_folder(&userfiles).await {
        Ok(Some(_)) => {
            println!("Top level folder already exists");
        }
        Ok(None) => {
            println!("Top level folder does not exist, creating...");
            create_top_level_folder(&userfiles, users_path).await?;
        }
        _ => {}
    }


    let mut cursor = userfiles.find(None, None).await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                println!("{:?}", document);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(database)
}

async fn find_top_level_folder(userfiles: &Collection<Document>) -> Result<Option<Document>, mongodb::error::Error> {
    let filter = doc! { "file_type": "top_level_folder" };
    let result = userfiles.find_one(filter, None).await?;
    Ok(result)
}

async fn create_top_level_folder(userfiles: &Collection<Document>, users_path: PathBuf) -> Result<(), mongodb::error::Error> {
    let top_level_folder = FileInfoRec {
        file_name: "files".into(),
        file_path: "files/".into(),
        is_dir: true,
        file_type: "top_level_folder".into(),
        size_in_b: 0,
        visibility: Visibility::Public,
        created_at: Utc::now(),
        uploaded_at: Utc::now(),
        description: "TopLevelFolder".into(),
        children: read_dir_recursive(users_path.join("files/"), String::from("files/")).map_or(None, |children| Some(children)),
    };
    let top_level_folder_serialized = to_bson(&top_level_folder).expect("Failed to serialize");
    if let Bson::Document(document) = top_level_folder_serialized {
        userfiles.insert_one(document, None).await.unwrap();
    }
    Ok(())
}
