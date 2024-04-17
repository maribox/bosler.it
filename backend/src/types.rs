use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;



#[derive(Error, Debug)]
pub enum AppError {
    #[error("Startup error: {0}")]
    StartupError(#[from] StartupError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    // Add more error variants as needed
    #[error("Other error: {0}")]
    OtherError(String),
}



#[derive(Error, Debug)]
pub enum StartupError {
    #[error("Starting the DB did not work:\n{reason}")]
    DbInitializationError {
        #[from]
        reason: mongodb::error::Error,
    },

    #[error("Binding the server to a port did not work:\n{reason}")]
    ServerBindError {
        ip_addr: std::net::IpAddr,
        port: i32,
        reason: String,
    },
}



#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfoRec {
    pub file_name: String,
    pub file_path: String,
    pub is_dir: bool,
    pub file_type: String,
    pub size_in_b: u64,
    pub visibility: Visibility,
    pub created_at: DateTime<Utc>,
    pub uploaded_at: DateTime<Utc>,
    pub description: String,
    pub children: Option<Vec<FileInfoRec>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Visibility {
    Public,
    Private,
}

