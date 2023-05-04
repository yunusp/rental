use rocket::{get, serde::json::Json, State};

use crate::{models::user_model::User, repo::user_repo::UserRepo};

#[get("/user/<uname>")]
pub async fn get_user(uname: String,  db: &State<UserRepo>) -> Option<Json<User>> {
    db.get_user(&uname).await.map(Json)
}
