use rocket::{fairing::{Kind, Fairing, Info}, Request, Response, http::Header};

// enable cors for rocket
pub struct Cors;
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            _request.headers().get("origin").next().unwrap(),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Set-Cookie, Access-Control-Allow-Headers, Origin, X-Requested-With, Accept, Authorization"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}