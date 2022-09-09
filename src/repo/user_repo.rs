use crate::models::user_model::User;
use bson::doc;
use dotenv;
use mongodb::{error::Error, results::InsertOneResult, Client, Collection};
use std::env;

pub struct UserRepo {
    pub col: Collection<User>,
}

impl UserRepo {
    pub async fn init() -> Self {
        dotenv::dotenv().ok();
        let uri = env::var("MONGOURI").expect("ERROR: no uri found");
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rental");
        let col: Collection<User> = db.collection("users");
        UserRepo { col }
    }

    ///Add 1 user to the database.
    ///
    /// `None` indicates a user is already present.
    pub async fn add_user(&self, user: User) -> Option<Result<InsertOneResult, Error>> {
        println!("Adding user");
        if self.is_duplicate(&user.uname).await {
            println!("found duplicate: {}", user.uname);
            return None;
        }
        let new_user = User {
            id: None,
            uname: user.uname,
            pass: user.pass,
        };
        Some(self.col.insert_one(new_user, None).await)
    }
    #[allow(dead_code)]
    pub async fn validate_user(&self, _user: User) -> Option<()> {
        Some(())
    }

    pub async fn get_user(&self, uname: &str) -> Option<User> {
        self.col
            .find_one(
                doc! {
                    "uname": uname.to_owned()
                },
                None,
            )
            .await
            .expect("Error fetching users.")
        // None
    }

    async fn is_duplicate(&self, uname: &String) -> bool {
        matches!(
            self.col
                .find_one(
                    doc!(
                        "uname": uname.to_owned()
                    ),
                    None,
                )
                .await
                .unwrap(),
            Some(_)
        )
    }
}
