
#![allow(non_snake_case)]
use actix_web::{web,Responder,HttpResponse};
use serde::Deserialize;
use mongodb::Database;
pub use mongodb::bson::{doc, Document};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
#[path = "../models/user_schema.rs"]
mod user_schema;
use user_schema::User;
use mongodb::error::Error;

#[derive(Deserialize)]
pub struct UserQueryDelete {
    pub name:String,
}
pub async fn deleteUser(db:web::Data<Database>, Info: web::Query<UserQueryDelete>)->impl Responder{
    let  collection = db.collection::<User>("Rust_Users");
    let filter = doc!{"name":Info.name.clone()};
    let _data = match collection.delete_one(filter,None).await {
        Ok(_data) => { 
            return  HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(201).unwrap())
            .body(format!("User deleted"))
        }, Err(err) => {
            return  HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(501).unwrap())
            .body(format!("internal server error : {}",err))
        }
    };//end of match statement}//end of handler
}

#[derive(Deserialize)]
pub struct UserQueryGet{
pub name:String,
} 
pub async fn getUser(db:web::Data<Database>, Info: web::Query<UserQueryGet>)->impl Responder {
    let collection = db.collection::<User>("Rust_Users");
    let filter = doc!{"name":Info.name.clone()};
    let data = match collection.find_one(filter, None).await {
        Ok(data)=> {
            data
        }, 
        Err(err)=> {
            return  HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(501).unwrap())
                .body(format!("could not post data, reason : {}",err))}
    };//end of match statement
    match data { Some(data) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(200).unwrap())
            .body(format!("Name : {}  \n  Email : {}  \n Age : {} ", data.name, data.email, data.age))}, None => {
                return  HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("X-Hdr", "sample"))
                    .status(StatusCode::from_u16(404).unwrap()).body(format!("No User with name {} exist in the database", Info.name))
                } 
        };//end of match statment
}//end of handler


#[derive(Deserialize)]
    pub struct UserDataUpdate{
    pub name:String,
    pub relese_date:Option<String>,
    pub phone:Option<String>,
    pub age:Option<i32>
}
pub async fn updateUser(db:web::Data<Database>, Info: web::Json<UserDataUpdate>)->impl Responder {
    let  collection = db.collection::<User>("Rust_Users");
    let User: Result<Option<User>, Error> = collection.find_one(doc!{"name":Info.name.clone()},None).await;
    let User: Option<User>  = match User {
        Ok(User) => User,
        Err(_err) => {
            return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(501).unwrap())
            .body(format!("internal server error"))
        }
    };//end of match
    let User = match User { 
        Some(User) => User, None => {
            return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(401).unwrap())
            .body(format!("no User with name {} exist",Info.name))
        }
    };//end of match statement
    let releasedate = if Info.relese_date == None{ User.email} else { Info.relese_date.clone().unwrap()};
    let phone = if Info.phone == None{ User.phone} else { Info.phone.clone().unwrap() };
    let age = if Info.age == None{ User.age} else { Info.age.unwrap() };
    let filter = doc!{"name":Info.name.clone()};
    let update = doc!{"$set":{"email":releasedate,"phone":phone,"age":age}};
    match collection.update_one(filter,update, None)
    .await{ Err(_err) => {
        return HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("X-Hdr", "sample"))
        .status(StatusCode::from_u16(501).unwrap())
        .body(format!("internal server error {} ",_err)) }, Ok(_ok) => {
            return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(201).unwrap())
            .body(format!("update finished"))    
        }
    }//end of match arm
}//end of handler
#[derive(Deserialize)]
pub struct UserData {
    pub name:String,
    pub email:String,
    pub phone:String,
    pub age:i32
}
pub async fn postUser(db:web::Data<Database>, Info: web::Json<UserData>)->impl Responder {
    let data = User{
        name:Info.name.clone(),
        email:Info.email.clone(),
        phone:Info.phone.clone(),
        age:Info.age.clone(),
     
    };
    let collection = db.collection::<User>("Rust_Users");
    match collection.insert_one(data, None).await {
        Ok(_ok)=> {     
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(200).unwrap())
            .body(format!("data posted")) 
        }, Err(err)=> {      
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(501).unwrap())
            .body(format!("could not post data, reason : {}", err))           
        }    
    } // end of match statement}
}  // end of handler