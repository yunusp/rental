mod controllers;
mod models;
mod repo;
use std::{collections::HashMap, sync::Mutex};

use controllers::car::{drop_car, get_car, get_cars, p_add_car, update_car};
use controllers::users::get_user;
use rental::{all_options, handle_null_cars, handle_null_images, CORS};
use repo::car_repo::CarRepo;
use rocket::fs::{relative, FileServer};
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
                get_car,
                update_car,
                get_user,
                drop_car,
                handle_null_cars,
                handle_null_images,
            ],
        )
        .mount("/public", FileServer::from(relative!("uploads")))
        .attach(CORS)
}
