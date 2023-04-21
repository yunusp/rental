use crate::{models::car_model::Car, repo::car_repo::CarRepo};
use rocket::{form::Form, get, http::Status, post, serde::json::Json, FromForm, State};

#[get("/cars")]
pub async fn get_cars(car_db: &State<CarRepo>) -> Json<Vec<Car>> {
    Json(car_db.get_all_cars().await)
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
