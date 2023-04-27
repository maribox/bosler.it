use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use actix_web::{get, HttpResponse, HttpServer, Responder, Result, web};
use chrono::{DateTime, Utc};
use crate::website::{AppState, data_repository};

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::types::{FileInfoRec, Visibility};

#[get("/files/data/{file_path:.*}")]
pub async fn get_file_data(path: web::Path<String>) -> Result<String> {
    let filepath = path.into_inner();
    Ok(format!("Filepath: {}!", filepath))
}

#[get("/files/data")]
pub async fn get_all_file_data(data: web::Data<AppState>) -> Result<HttpResponse> {
    let users_path = PathBuf::from(&data.users_path).join("files");

    let files: Vec<FileInfoRec>  = data_repository::read_dir_recursive(users_path, String::from(""))?;
    let json_data = to_string(&files).unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(json_data))
}


