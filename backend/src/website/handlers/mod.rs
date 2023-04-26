use std::ffi::OsStr;
use std::fs;
use std::path::{ PathBuf};
use actix_web::{get, web, HttpServer, Result, Responder, HttpResponse};
use chrono::{DateTime, Utc};
use crate::website::AppState;

use serde::{Serialize, Deserialize};
use serde_json::{to_string};


#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    name: String,
    is_dir: bool,
    file_type: String,
    size_in_b: u64,
    visibility: Visibility,
    created_at: DateTime<Utc>,
    uploaded_at: DateTime<Utc>,
    download_url: String,
    description: String,
    children: Option<Vec<FileInfo>>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonFileInfo {
    created_at: DateTime<Utc>,
    uploaded_at: DateTime<Utc>,
    download_url: String,
    description: String,
    visibility: Visibility,
}

#[derive(Serialize, Deserialize)]
enum Visibility {
    Public,
    Private,
}

#[get("/files/data/{file_path:.*}")]
pub async fn get_file_data(path: web::Path<String>) -> Result<String> {
    let filepath = path.into_inner();
    Ok(format!("Filepath: {}!", filepath))
}

#[get("/files/data")]
pub async fn get_all_file_data(data: web::Data<AppState>) -> Result<HttpResponse> {
    let users_path = PathBuf::from(&data.users_path).join("files");

    let files: Vec<FileInfo>  = read_dir_recursive(users_path)?;
    let json_data = to_string(&files).unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(json_data))
}


fn read_dir_recursive(user_path: PathBuf) -> Result<Vec<FileInfo>> {
    let file_paths = fs::read_dir(user_path.to_owned())?;
    let mut files_recursive = Vec::new();
    for file in file_paths {
        let metadata = file.as_ref().unwrap().metadata()?;
        let filepath = file.unwrap().path();
        let name = filepath.file_name().unwrap().to_str().unwrap().to_string();
        let is_dir = filepath.is_dir();
        let children = if is_dir {
            let children = match read_dir_recursive(filepath.clone()) {
                Ok(children) => children,
                Err(_) => Vec::new(),
            };
            Some(children)
        } else {
            None
        };
        let file_info = FileInfo {
            name,
            is_dir,
            file_type: filepath.extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .to_owned(),
            size_in_b: metadata.len(),
            visibility: Visibility::Public,
            created_at: metadata.created()?.into(),
            uploaded_at: Default::default(),
            download_url: filepath.to_string_lossy().parse()?,
            description: "".to_string(),
            children,
        };
        files_recursive.push(file_info);
    }
    Ok(files_recursive)
}


