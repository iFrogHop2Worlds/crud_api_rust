

pub(crate) use serde::{Deserialize,Serialize};
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use mongodb::Client;
use bson::doc;#[derive(Deserialize,Serialize)]

pub struct User {
    pub name:String,
    pub email:String,
    pub phone:String,
    pub age:i32
}

pub async fn create_name_unique(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc!{"name":1})
        .options(options)
        .build();client
        .database("Tsuga-Genie")  //name of our database
        .collection::<User>("Rust_Users") //name of our collection
        .create_index(model, None)
        .await
        .expect("error creating index!");
}