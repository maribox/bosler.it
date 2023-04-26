mod handlers;
mod data_repository;
pub(crate) mod utils;

use actix_web::{web, HttpResponse};
use actix_web_lab::web::spa;
use std::env;
use std::path::PathBuf;
use crate::website::utils::init_path;

pub struct AppState {
    users_path: String
}

const FRONTEND_PATH_DEFAULT: &str = "/home/marius/server/bosler.it/frontend";
const USERS_PATH_DEFAULT: &str = "/home/marius/server/bosler.it/users";

pub fn website_spa() -> impl actix_web::dev::HttpServiceFactory {
    let frontend_path = init_path(env::args().nth(1), FRONTEND_PATH_DEFAULT);
    spa()
        .index_file(frontend_path.join("dist").join("index.html").to_string_lossy().into_owned())
        .static_resources_mount("/assets")
        .static_resources_location(frontend_path.join("dist").join("./assets").to_string_lossy().into_owned())
        .finish()
}

pub fn api_scope() -> actix_web::Scope {
    let user_path = init_path(env::args().nth(2), USERS_PATH_DEFAULT);
    web::scope("/api")
        .service(handlers::get_file_data)
        .service(handlers::get_all_file_data)
        .app_data(web::Data::new(AppState {
            users_path: user_path.to_string_lossy().into_owned()
        }))
}
