mod controllers;
mod interfaces;
mod models;
mod repo;
use std::{collections::HashMap, sync::Mutex};

use rocket::{launch, routes, options};
use rocket_dyn_templates::Template;

use crate::controllers::{
    index::g_index,
    signin::{g_sign_in, p_sign_in},
    signup::{g_sign_up, p_sign_up},
};
use crate::repo::user_repo::UserRepo;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

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
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}


#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let db = UserRepo::init().await;
    let ctx: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(db)
        .manage(ctx)
        .mount(
            "/",
            routes![g_index, g_sign_in, p_sign_in, g_sign_up, p_sign_up, all_options],
        )
        .attach(Template::fairing())
        .attach(CORS)
}
