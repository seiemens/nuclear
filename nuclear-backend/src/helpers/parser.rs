use std::env;
use mongodb::bson::{oid::ObjectId};
use rocket::{serde::json::Json, fs::TempFile, Error};

use crate::{models::{User, File}};
use super::{token, endecr};

/// Slams the user data through a model to make it easier to use.
pub fn parse_user(u: Json<User>) -> Result<User, Error> {
    
    // generate a token if no token has been supplied.
    let token:String;
    if u.auth_token == None {
        token = token::generate(64);
    }else{
        token = u.auth_token.clone().unwrap();
    }

    let data = User {
        _id: u._id.to_owned(),
        name: u.name.to_owned(),
        password: endecr::encrypt(u.password.to_owned()),
        email: u.email.to_owned(),
        role:u.role.to_owned(),
        auth_token:Some(token.to_owned()),
    };
    return Ok(data);
}

/// Parses the file data into an insertable type
pub async fn parse_file(file: &mut TempFile<'_>, owner:String) -> File {
    
    // set in here and not in `connector.rs` since its only called here.
    let path = env::var("STOREPATH").expect("STOREPATH HAS TO BE SET");

    let name = file.raw_name().unwrap().as_str().unwrap();
    let extension = file.content_type().unwrap().0.extension().unwrap().as_str();
    let fullname = format!("{name}.{extension}");

    // make a persistent file out of the "temporary" one.
    let path = format!("{path}/{fullname}");
    let _ = file.persist_to(path.clone()).await;

    let data = File {
        _id: Some(ObjectId::new()),
        name: Some(fullname),
        path: Some(path),
        owned_by: Some(owner.to_owned()),
        size: Some(file.len().to_owned()),
    };

    data
}