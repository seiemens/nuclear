/*
- upload
- delete
- share (maybe?)
*/

use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, http::{Status, CookieJar}, State, form::Form, fs::TempFile};
use crate::{models::{User, File, Upload}, helpers::{endecr, token, cookies::{get_cookie_value, cookie}, parser::parse_file}, data::connector::Connector};


#[get("/files")]
pub async fn fetch_files(jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Json<Vec<File>>,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
    .await;     
    if auth.is_ok() {
        let files = db.fetch_files(auth.unwrap().auth_token.unwrap()).await;
        return Ok(Json(files.unwrap()));
    }
    else{
        return Err(Status::ImATeapot);
    }
}

#[post("/upload", data="<f>", format="multipart/form-data")]
pub async fn upload_file(mut f:Form<Upload<'_>>, jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Status,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit"))).await;
    if auth.is_ok() {
        let parsed = parse_file(&mut f.file, auth.unwrap().auth_token.unwrap());
        let file = db.upload_file(parsed.await).await;
        println!("{:?}", file);

        return Ok(Status::Ok);
    }
    return Err(Status::ImATeapot);
}

#[post("/delete", data="<f>")]
pub async fn delete_file(f:Json<ObjectId>, jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Status,Status>{
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