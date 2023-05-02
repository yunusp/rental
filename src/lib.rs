use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::{get, options, Request, Response};
use sha2::{self, Digest, Sha256};

pub fn sha256sum(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:X}", hasher.finalize()) //:X = hexadecimal uppercase
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[allow(clippy::let_unit_value)]
#[options("/<_..>")]
pub fn all_options() {
    /* Intentionally left empty */
}

// need the next two methods to future proof just in case
// someone decides to take a funny username

#[get("/public/image-null")]
pub async fn handle_null_images() -> Status {
    Status::NotFound
}
#[get("/public/car-null")]
pub async fn handle_null_cars() -> Status {
    Status::NotFound
}
