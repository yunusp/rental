mod controllers;
mod models;
mod repo;
use std::{collections::HashMap, sync::Mutex};

use controllers::car::{get_car, get_cars, p_add_car};
use rental::{all_options, CORS};
use repo::car_repo::CarRepo;
use rocket::{launch, routes};

use crate::controllers::{signin::p_sign_in, signup::p_sign_up};
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
                p_sign_up,
                all_options,
                get_cars,
                p_add_car,
                get_car
            ],
        )
        .attach(CORS)
}
