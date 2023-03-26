use std::{collections::HashMap, sync::Mutex};

use bson::doc;
use rental::sha256sum;
use rocket::{
    form::Form, http::Status, post, FromForm, State,
};
use crate::repo::user_repo::UserRepo;

#[derive(FromForm, Debug)]
pub struct SignInForm {
    uname: String,
    pass: String,
}

#[post("/signin", data = "<data>")]
pub async fn p_sign_in(
    data: Form<SignInForm>,
    _ctx: &State<Mutex<HashMap<String, String>>>,
    db: &State<UserRepo>,
) -> Status {
    let hash = sha256sum(&data.pass);
    match db.get_user(&data.uname).await {
        Some(user) => {
            if user.pass == hash {
                Status::Ok
            } else {
                Status::Unauthorized
            }
        }
        None => {
            Status::Forbidden
        }
    }
}
