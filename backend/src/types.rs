use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
