use futures::TryStreamExt;
use mongodb::{Client, options::ClientOptions};
use crate::types::FileInfoRec;

mod types;

async fn fetch_file_info_recs() -> Result<Vec<FileInfoRec>, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options)?;
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    let db = client.database("Website");
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    let collection = db.collection::<FileInfoRec>("Userfiles");

    let mut cursor = collection.find(None, None).await?;
    while let Some(file) = cursor.try_next().await? {
        println!("title: {}", file.file_name);
    }


    Ok(cursor.try_collect().await?)
}

#[tokio::main]
async fn main() {
    match fetch_file_info_recs().await {
        Ok(file_info_records) => println!("{:#?}", file_info_records),
        Err(e) => eprintln!("Error fetching file info records: {}", e),
    }
}
