use futures::TryStreamExt;
use mongodb::{bson, Client};
use crate::types::FileInfoRec;

mod types;

async fn fetch_file_info_recs() -> Result<Vec<FileInfoRec>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("Website");
    let collection = database.collection::<FileInfoRec>("Userfiles");

    let cursor = collection.find(None, None).await?;
    let file_info_records= cursor.try_collect().await;

    file_info_records
}

#[tokio::main]
async fn main() {
    match fetch_file_info_recs().await {
        Ok(file_info_records) => println!("{:#?}", file_info_records),
        Err(e) => eprintln!("Error fetching file info records: {}", e),
    }
}
