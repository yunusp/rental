use crate::models::car_model::Car;
use bson::{doc, oid::ObjectId};
use dotenv;
use futures::TryStreamExt;
use mongodb::{error::Error, results::InsertOneResult, Client, Collection};
use std::{env, str::FromStr};

pub struct CarRepo {
    pub col: Collection<Car>,
}
impl CarRepo {
    pub async fn init() -> Self {
        dotenv::dotenv().ok();
        let uri = env::var("MONGOURI").expect("ERROR: no uri found");
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rental");
        let col: Collection<Car> = db.collection("cars");
        CarRepo { col }
    }

    /// None indicates duplicate.
    pub async fn add_car(&self, car: Car) -> Option<Result<InsertOneResult, Error>> {
        if self.is_duplicate(&car.number).await {
            println!("found duplicate: {}", car.number);
            return None;
        }
        let new_car = car.clone();
        Some(self.col.insert_one(new_car, None).await)
    }

    pub async fn get_all_cars(&self) -> Vec<Car> {
        self.col
            .find(None, None)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap_or_default()
    }

    pub async fn get_car(&self, id: &str) -> Option<Car> {
        self.col
            .find_one(
                doc! {
                    "_id": ObjectId::from_str(id).expect("Could not construct object id")
                },
                None,
            )
            .await
            .expect("Error fetching car.")
    }

    async fn is_duplicate(&self, number: &str) -> bool {
        matches!(
            self.col
                .find_one(
                    doc!(
                        "number": number.to_owned()
                    ),
                    None,
                )
                .await
                .unwrap(),
            Some(_)
        )
    }
}
