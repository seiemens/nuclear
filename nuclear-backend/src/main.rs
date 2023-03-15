#[macro_use] extern crate rocket;

use std::vec;
use data::connector::Connector;
use helpers::cors;
use endpoints::user::{auth_session, new_user, login, logout, delete_user};

mod data;
mod models;
mod helpers;
mod endpoints;

#[launch]
async fn rocket() -> _ {
    let db = Connector::init().await;

    rocket::build()
    .manage(db)
    .attach(cors::Cors)
    .mount("/", routes![
        login, logout, auth_session, new_user, delete_user, preflight
    ])
}

/*
----- CORS RELATED -----
*/

#[options("/<_..>")]
fn preflight() {
    // left empty so preflight is happy
}