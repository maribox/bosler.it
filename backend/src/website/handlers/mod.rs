pub(crate) mod broadcast;
use self::broadcast::Broadcaster;
use std::path::PathBuf;
use actix_web::{Error, get, post, HttpRequest, HttpResponse, Responder, Result, web};
use actix_web_lab::{extract::Path, respond::Html};
use crate::website::{data_repository};
use serde_json::to_string;
use crate::ServerState;
use crate::types::{FileInfoRec};

#[get("/files/data/{file_path:.*}")]
pub async fn get_file_data(path: web::Path<String>) -> Result<String> {
    let filepath = path.into_inner();
    Ok(format!("Filepath: {}!", filepath))
}

#[get("/files/data")]
pub async fn get_all_file_data(data: web::Data<ServerState>) -> Result<HttpResponse> {
    let users_path = PathBuf::from(&data.users_path).join("files");

    let files: Vec<FileInfoRec>  = data_repository::read_dir_recursive(users_path, String::from(""))?;
    let json_data = to_string(&files).unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(json_data))
}

//alarm
#[get("/huhn")]
pub async fn index() -> impl Responder {
    Html(include_str!("index.html").to_string())
}

#[get("/events")]
pub async fn event_stream(data: web::Data<ServerState>) -> impl Responder {
    data.broadcaster.new_client().await
}

#[post("/broadcast/{msg}")]
pub async fn broadcast_msg(
    data: web::Data<ServerState>,
    Path((msg,)): Path<(String,)>,
) -> impl Responder {
    data.broadcaster.broadcast(&msg).await;
    HttpResponse::Ok().body("msg sent")
}


