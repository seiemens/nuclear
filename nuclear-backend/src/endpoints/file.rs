/*
- upload
- delete
- share (maybe?)
*/

use rocket::{serde::json::Json, http::{Status, CookieJar, Cookie}, State};
use crate::{models::{User, File}, helpers::{endecr, token, cookies::{get_cookie_value, cookie}}, data::connector::Connector};


#[get("/files")]
pub async fn fetch_files(jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Status,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
    .await;     
    if auth.is_ok() {

    }
    return Ok(Status::Ok);
}

#[post("/upload", data="<f>")]
pub async fn upload_file(f:Json<File>, jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Status,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
    .await;    

    if auth.is_ok() {

    }
    return Ok(Status::Ok);
}

#[get("/delete")]
pub async fn delete_file(jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Status,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
    .await;    

    if auth.is_ok() {
    }
    return Ok(Status::Ok);
}


#[get("/share")]
pub async fn gen_link(jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Json<String>,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
    .await;    
    return Ok(Json("aw".to_string()));
}