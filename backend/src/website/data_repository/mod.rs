use std::path::PathBuf;
use std::fs;
use std::ffi::OsStr;
use chrono::Utc;
use futures::StreamExt;
use mongodb::bson::{Bson, doc, Document, to_bson};
use mongodb::{Client, Collection, Database};
use mongodb::options::ClientOptions;
use crate::types::{AppError, FileInfoRec, StartupError, Visibility};



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


pub async fn init_db(users_path: PathBuf) -> Result<Database, AppError> {
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
