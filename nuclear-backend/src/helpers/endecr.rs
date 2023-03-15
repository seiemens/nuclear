extern crate argon2;
extern crate dotenv;
use std::env;

use argon2::Config;
use dotenv::dotenv;

/*
------ NOTE -----
This Algorithm uses a so-called 'salt' string. it can be set in the .env file.
*/
pub fn encrypt(pw: String) -> String {
    dotenv().ok();
    let salt = env::var("SALT").expect("SALT REQUIRED");

    // argon2 config could be altered for more security
    let config = Config::default();

    let hash = argon2::hash_encoded(pw.as_bytes(), salt.as_bytes(), &config).unwrap();

    return hash;
}