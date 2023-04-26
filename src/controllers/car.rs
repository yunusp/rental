use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use std::{env, fs::{File, self}, io::Write, path::PathBuf};

use crate::{models::car_model::Car, repo::car_repo::CarRepo};
use rocket::{
    delete, form::Form, get, http::Status, patch, post, serde::json::Json, FromForm, State,
};

#[get("/cars")]
pub async fn get_cars(car_db: &State<CarRepo>) -> Json<Vec<Car>> {
    Json(car_db.get_all_cars().await)
}

#[get("/cars/<carid>")]
pub async fn get_car(carid: String, car_db: &State<CarRepo>) -> Option<Json<Car>> {
    if let Some(car) = car_db.get_car(&carid).await {
        return Some(Json(car));
    } else {
        return None;
    }
}

#[derive(FromForm)]
pub struct CarAddForm {
    name: String,
    brand: String,
    number: String,
    price: u64,
    yop: u16,
    iat: String,
    ito: String,
    picture: String,
    desc: Option<String>,
    owner_id: String,
}

#[post("/cars", data = "<data>")]
pub async fn p_add_car(car_db: &State<CarRepo>, data: Form<CarAddForm>) -> Status {
    dotenv().unwrap_or_else(|_| PathBuf::default());
    let bytes = general_purpose::STANDARD.decode(&data.picture).unwrap();
    let file_dir = env::var("UPLOAD_PATH").unwrap();
    let file_id = &format!("car-{}", data.number);
    let file_name = &format!("{}/{}", &file_dir, &file_id);
    let new_car = Car {
        id: None,
        owner_id: Some(data.owner_id.clone()),
        borrower_id: None,
        brand: data.brand.to_owned(),
        iat: data.iat.to_owned(),
        ito: data.ito.to_owned(),
        number: data.number.to_owned(),
        name: data.name.to_owned(),
        price: data.price,
        yop: data.yop,
        dt: 600, // ! also change this asap
        picture: file_id.to_owned(),
        desc: data.desc.clone(),
    };

    match car_db.add_car(new_car).await {
        Some(x) => {
            x.unwrap();
            let mut handle = File::create(file_name).unwrap();
            handle.write_all(&bytes).unwrap();
            Status::Created
        }
        None => Status::Unauthorized,
    }
}

#[patch("/cars/<carid>", data = "<b_id>")]
pub async fn update_car(carid: String, b_id: Form<String>, car_db: &State<CarRepo>) -> Status {
    if let Some(_) = car_db.get_car(&carid).await {
        car_db.set_borrower_id(&carid, &b_id).await;
        return Status::Ok;
    } else {
        return Status::InternalServerError;
    }
}

#[delete("/cars/<carid>")]
pub async fn drop_car(carid: String, car_db: &State<CarRepo>) -> Status {
    let car = car_db.get_car(&carid).await.unwrap();
    if matches!(car_db.drop_car(&carid).await, true) {
        fs::remove_file(&format!("uploads/car-{}", car.number)).unwrap();
        Status::Ok
    } else {
        Status::InternalServerError
    }
}
