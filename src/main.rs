mod controllers;
mod interfaces;
mod models;
mod repo;
use std::{collections::HashMap, sync::Mutex};

use rental::{all_options, CORS};
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

use crate::controllers::{
    signin::p_sign_in,
    signup::{g_sign_up, p_sign_up},
};
use crate::repo::user_repo::UserRepo;

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let db = UserRepo::init().await;
    let ctx: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(db)
        .manage(ctx)
        .mount("/", routes![p_sign_in, g_sign_up, p_sign_up, all_options])
        .attach(Template::fairing())
        .attach(CORS)
}
