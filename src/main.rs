mod controllers;
mod models;
mod repo;
mod interfaces;
use std::{collections::HashMap, sync::Mutex};

use rocket::{launch, routes};
use rocket_dyn_templates::Template;

use crate::controllers::{
    index::g_index,
    signin::{g_sign_in, p_sign_in},
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
        .mount(
            "/",
            routes![g_index, g_sign_in, p_sign_in, g_sign_up, p_sign_up],
        )
        .attach(Template::fairing())
}
