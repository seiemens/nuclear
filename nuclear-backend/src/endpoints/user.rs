use argon2::Error;
use mongodb::results::{InsertOneResult, DeleteResult};
use rocket::{serde::json::Json, http::{Status, CookieJar, Cookie}, State};

use crate::{models::User, helpers::{endecr, token, cookies::{get_cookie_value, cookie}}, data::connector::Connector};

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

/*
----- Management Section -----
*/

#[post("/user/new", data="<u>")]
pub async fn new_user(
    u:Json<User>,
    db: &State<Connector>
) -> Result<Json<InsertOneResult>, Status> {
    let data = parse_user(u).unwrap();
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::ImATeapot),
    }
}

#[post("/user/delete", data="<u>")]
pub async fn delete_user(
    u:Json<User>,
    db: &State<Connector>,
    jar: &CookieJar<'_>
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

#[post("/login", data="<u>")]
pub async fn login(
    u:Json<User>,
    db: &State<Connector>,
    jar: &CookieJar<'_>
) -> Result<Json<User>, Status> {
    let data = parse_user(u).unwrap();
    let user = db.get_user(data.clone()).await;
    println!("{:?}", user);

    if let Ok(None) = user {
        return Err(Status::ImATeapot);
    } else {
        // and return a biscuit if it does.
        jar.add(cookie(
            String::from("auth"),
            String::from(user.clone().unwrap().unwrap().auth_token.unwrap()),
        ));
        return Ok(Json(user.unwrap().unwrap()));
    }
}

#[get("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Result<Status,Status>{
    jar.remove(Cookie::named("auth_biscuit"));
    return Ok(Status::Ok);
}

#[get("/auth")]
pub async fn auth_session(
    db: &State<Connector>,
    jar: &CookieJar<'_>
) -> Result<Json<User>, Json<bool>> {
    let auth = db
        .verify_auth(get_cookie_value(jar, String::from("auth_biscuit")))
        .await;
    if auth.is_ok() {
        return Ok(Json(auth.unwrap()));
    } else {
        return Err(Json(false));
    }
}


