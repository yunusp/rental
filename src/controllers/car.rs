use std::str::FromStr;

use crate::{models::car_model::Car, repo::car_repo::CarRepo};
use bson::oid::ObjectId;
use rocket::{form::Form, get, http::Status, post, serde::json::Json, FromForm, State};

#[get("/cars")]
pub async fn get_cars(car_db: &State<CarRepo>) -> Json<Vec<Car>> {
    Json(car_db.get_all_cars().await)
}

#[derive(FromForm)]
pub struct CarAddForm {
    pub name: String,
    pub brand: String,
    pub number: String,
    pub price: u64,
    pub yop: u16,
    pub iat: String,
    pub ito: String,
    pub picture: String,
    pub desc: Option<String>,
    owner_id: String,
}

#[post("/cars", data = "<data>")]
pub async fn p_add_car(car_db: &State<CarRepo>, data: Form<CarAddForm>) -> Status {
    // let oid = get_oid_by_uname(data.owner_id);
    let new_car = Car {
        id: None,
        // owner_id: Some(oid),
        owner_id: Some(data.owner_id.clone()), // ! change this ASAP
        borrower_id: None,
        brand: data.brand.to_owned(),
        iat: data.iat.to_owned(),
        ito: data.ito.to_owned(),
        number: data.number.to_owned(),
        name: data.name.to_owned(),
        price: data.price,
        yop: data.yop,
        dt: 600, // ! also change this asap
        picture: data.picture.to_owned(),
        desc: data.desc.clone(),
    };

    match car_db.add_car(new_car).await {
        Some(x) => {
            x.unwrap();
            Status::Created
        }
        None => Status::Unauthorized,
    }
}
