use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::time::{Duration};

/*
----- BISCUIT GENERATOR -----
-> We love cookie clicker, don't we?
*/

/// generate a new cookie based on the parameters
pub fn cookie(name: String, value: String) -> Cookie<'static> {
    let cookie = Cookie::build(name, value)
    .path("/")
    .max_age(Duration::hours(3))
    .secure(true)
    .http_only(true)
    .same_site(SameSite::None)
    .finish(); // Setting the expiry date to 'None' sets it to expire when the session gets closed.
    println!("{:?}", cookie);
    return cookie;
}

/// Used to extract value from cookie.
pub fn get_cookie_value(jar: &CookieJar<'_>, name: String) -> String {
    let c = jar.get(&name);

    if c.is_some() {
        // since cookies are stored as a string in the format of "key=value"
        // they have to be split into an array.
        return String::from(jar.get(&name).map(|cookie| cookie.value()).unwrap());
    }else {
        return String::from("");
    }
}