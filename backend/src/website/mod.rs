mod handlers;
pub(crate) mod data_repository;
pub(crate) mod utils;

use actix_web::{web};
use actix_web_lab::web::spa;
use std::env;
use crate::website::utils::init_path;



const FRONTEND_PATH_DEFAULT: &str = "/home/marius/server/bosler.it/frontend";

pub fn website_spa() -> impl actix_web::dev::HttpServiceFactory {
    let frontend_path = init_path(env::args().nth(1), FRONTEND_PATH_DEFAULT);
    spa()
        .index_file(frontend_path.join("dist").join("index.html").to_string_lossy().into_owned())
        .static_resources_mount("/assets")
        .static_resources_location(frontend_path.join("dist").join("./assets").to_string_lossy().into_owned())
        .finish()
}

pub fn api_scope() -> actix_web::Scope {
    web::scope("/api")
        .service(handlers::get_file_data)
        .service(handlers::get_all_file_data)
}
