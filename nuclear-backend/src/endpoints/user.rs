use mongodb::results::{InsertOneResult, DeleteResult};
use rocket::{serde::json::Json, http::{Status, CookieJar, Cookie}, State};

use crate::{models::User, helpers::{cookies::{get_cookie_value}, parser::parse_user}, data::connector::Connector};

/*
----- Management Section -----
-> INFO:
    "match {}" -> is equal to a Switch() statement in C#
*/

/// create a new user
#[post("/user/new", data="<u>")]
pub async fn new_user(
    u:Json<User>,
    db: &State<Connector>
) -> Result<Json<InsertOneResult>, Status> {
    // parse the request data into a processable model
    let data = parse_user(u).unwrap();
    let user_detail = db.create_user(data).await;
    
    // rust "switch" case: 
    //check if the user is valid or not and return based on the Outcome.
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::ImATeapot),
    }
}

/// ADMIN ROUTE - Delete an existing user from the DB
#[post("/user/delete", data="<u>")]
pub async fn delete_user(
    u:Json<User>,
    db: &State<Connector>,
    _jar: &CookieJar<'_>
) -> Result<Json<DeleteResult>, Status> {
    let data = parse_user(u).unwrap();
    let user_detail = db.delete_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::ImATeapot),
    }}

/* 
----- Session Related Section -----
*/

/// login a user / create a cookie.
#[post("/login", data="<u>")]
pub async fn login(
    u:Json<User>,
    db: &State<Connector>,
    _jar: &CookieJar<'_>
) -> Result<Json<User>, Status> {
    let data = parse_user(u).unwrap();
    let user = db.get_user(data.clone()).await;
    println!("{:?}", user);

    if let Ok(None) = user {
        return Err(Status::ImATeapot);
    } else {
        return Ok(Json(user.unwrap().unwrap()));
    }
}

/// remove the cookie.
#[get("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Result<Status,Status>{
    jar.remove(Cookie::named("auth_biscuit"));
    return Ok(Status::Ok);
}

/// used on page navigation to ensure the user is still logged in.
#[get("/auth")]
pub async fn auth_session(
    db: &State<Connector>,
    jar: &CookieJar<'_>
) -> Result<Json<User>, Json<bool>> {
    let auth = db
        .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
        .await;
    // if a false is returned, the auth() function in the frontend will redirect to login
    if auth.is_ok() {
        return Ok(Json(auth.unwrap()));
    } else {
        return Err(Json(false));
    }
}


