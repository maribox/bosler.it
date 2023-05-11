use std::path::PathBuf;
use std::time::Duration;
use actix_web::{Error, get, HttpRequest, HttpResponse, Result, web};
use actix_web::http::header::{CacheControl, CacheDirective, ContentType};
use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use futures::stream::{};
use futures::StreamExt;
use crate::website::{data_repository};

use serde_json::to_string;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
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

#[get("/sse")]
pub(crate) async fn sse() -> Result<HttpResponse, Error> {
    let mut interval = interval(Duration::from_secs(1));
    let res = HttpResponse::build(ContentType::parse_flexible("text/event-stream")?)
        .header(CacheControl(vec![
            CacheDirective::NoCache,
            CacheDirective::Private,
        ]))
        .streaming(
            actix_web::web::streaming::Body::wrap_stream(async_stream::stream! {
                let mut i = 0;
                loop {
                    let _ = interval.tick().await;
                    let message = format!("data: hello from server ---- [{}]\n\n", i);
                    yield Ok::<_, actix_web::Error>(Bytes::from(message));
                    i += 1;
                }
            })
        );
    Ok(res)
}
