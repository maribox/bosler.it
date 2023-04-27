use mongodb::{bson, bson::doc, Client};
use mongodb::error::Result;
use std::path::PathBuf;
use std::fs;
use std::ffi::OsStr;
use crate::types::{FileInfoRec, Visibility};



pub fn read_dir_recursive(user_path: PathBuf, relative_path: String) -> actix_web::Result<Vec<FileInfoRec>> {
    let file_paths = fs::read_dir(user_path.to_owned())?;
    let mut files_recursive = Vec::new();
    for file in file_paths {
        let metadata = file.as_ref().unwrap().metadata()?;
        let filepath = file.unwrap().path();
        let name = filepath.file_name().unwrap().to_str().unwrap().to_string();
        let is_dir = filepath.is_dir();
        let children = if is_dir {
            let children = match read_dir_recursive(filepath.clone(), relative_path.clone() + &name + "/") {
                Ok(children) => children,
                Err(_) => Vec::new(),
            };
            Some(children)
        } else {
            None
        };
        let file_info = FileInfoRec {
            file_name: name.clone(),
            file_path: relative_path.clone() + &name,
            is_dir,
            file_type: filepath.extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .to_owned(),
            size_in_b: metadata.len(),
            visibility: Visibility::Public,
            created_at: metadata.created()?.into(),
            uploaded_at: Default::default(),
            description: "".to_string(),
            children,
        };
        files_recursive.push(file_info);
    }
    Ok(files_recursive)
}
