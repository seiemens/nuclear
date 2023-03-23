/*
- upload
- delete
- share (maybe?)
*/

use std::{str::FromStr, path::Path};

use mongodb::{results::{DeleteResult, InsertOneResult}, bson::oid::ObjectId};
use rocket::{serde::json::Json, http::{Status, CookieJar, Cookie}, State, form::Form, fs::NamedFile};
use crate::{models::{User, File, Upload}, helpers::{endecr, token, cookies::{get_cookie_value, cookie}, parser::parse_file}, data::connector::Connector};


#[get("/files")]
pub async fn fetch_files(jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Json<Vec<File>>,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
    .await;     
    if auth.clone().is_ok() {
        let files = db.fetch_files(auth.unwrap().auth_token.unwrap()).await;
        return Ok(Json(files.unwrap()));
    }
    else{
        return Err(Status::ImATeapot);
    }
}

#[post("/upload", data="<f>", format="multipart/form-data")]
pub async fn upload_file(mut f:Form<Upload<'_>>, jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Json<InsertOneResult>,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit"))).await;
    if auth.is_ok() {
        let parsed = parse_file(&mut f.file, auth.unwrap().auth_token.unwrap());
        let file = db.upload_file(parsed.await).await;
        println!("{:?}", file);

        return Ok(Json(file.unwrap()));
    }
    return Err(Status::ImATeapot);
}

#[post("/delete", data="<f>")]
pub async fn delete_file(f:Json<File>, jar: &CookieJar<'_>, db: &State<Connector>) -> Result<Json<DeleteResult>,Status>{
    let auth = db
    .verify_auth(get_cookie_value(jar, String::from("auth_biscuit"))).await;
    if auth.is_ok() {
        let file = db.delete_file(f._id.unwrap()).await;
        return Ok(Json(file.unwrap()));
    }
    else {
        return Err(Status::ImATeapot);
    }
}


#[get("/share/<id>")]
pub async fn gen_link(db: &State<Connector>, id:String) -> Result<NamedFile, Status>{
    let file = db.get_file(ObjectId::from_str(&id).unwrap()).await;
    let f = NamedFile::open(Path::new(&file.unwrap().path.unwrap())).await;
    println!("{:?}", f);
    match f {
        Ok(prepared_file) => return Ok(prepared_file),
        Err(_f) => return Err(Status::ImATeapot)
    }
}