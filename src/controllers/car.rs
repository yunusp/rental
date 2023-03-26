use crate::{models::car_model::Car, repo::car_repo::CarRepo};
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
}

#[post("/cars", data = "<data>")]
pub async fn add_car(car_db: &State<CarRepo>, data: Form<CarAddForm>) -> Status {
    let new_car = Car {
        id: None,
        owner_id: None, // ! change this ASAP
        borrower_id: None,
        brand: data.brand.to_owned(),
        iat: data.iat.to_owned(),
        ito: data.ito.to_owned(),
        number: data.number.to_owned(),
        name: data.name.to_owned(),
        price: data.price,
        yop: data.yop,
        dt: 600, // ! also change this asap
        picture: "imageplaceholder.png".to_owned(),
        desc: data.desc.clone(),
    };

    match car_db.add_car(new_car).await {
        Some(x) => {
            x.unwrap();
            return Status::Ok;
        }
        None => return Status::Forbidden,
    }

}
