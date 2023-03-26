mod controllers;
mod interfaces;
mod models;
mod repo;
use std::{collections::HashMap, sync::Mutex};

use controllers::car::{add_car, get_cars};
use rental::{all_options, CORS};
use repo::car_repo::CarRepo;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

use crate::controllers::{
    signin::p_sign_in,
    signup::{g_sign_up, p_sign_up},
};
use crate::repo::user_repo::UserRepo;

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let user_db = UserRepo::init().await;
    let car_db = CarRepo::init().await;

    let ctx: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(user_db)
        .manage(car_db)
        .manage(ctx)
        .mount(
            "/",
            routes![
                p_sign_in,
                g_sign_up,
                p_sign_up,
                all_options,
                get_cars,
                add_car
            ],
        )
        .attach(Template::fairing())
        .attach(CORS)
}
