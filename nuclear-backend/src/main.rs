#[macro_use] extern crate rocket;

use std::vec;
use data::connector::Connector;
use helpers::cors;
use endpoints::user::{auth_session, new_user, login, logout, delete_user};
use crate::endpoints::file::{fetch_files, delete_file, upload_file, gen_link};

mod data;
mod models;
mod helpers;
mod endpoints;

#[launch]
async fn rocket() -> _ {
    let db = Connector::init().await;

    rocket::build()
    .manage(db) // make the DB accessible to the whole application
    .attach(cors::Cors) // attach the CORS Fairing
    .mount("/", routes![
        login, logout, auth_session, new_user, delete_user,fetch_files,delete_file,upload_file,gen_link, preflight
    ])
}

/*
----- CORS RELATED -----
*/

#[options("/<_..>")]
fn preflight() {
    // left empty so preflight is happy
}