use crate::{models::user_model::User, repo::user_repo::UserRepo};
use base64::{engine::general_purpose, Engine as _};
use bson::doc;
use chrono::{self, Utc};
use dotenv::dotenv;
use rental::sha256sum;
use rocket::{form::Form, post, FromForm, State, http::Status};
use std::{collections::HashMap, env, fs::File, io::Write, path::PathBuf, sync::Mutex};

#[derive(Debug, FromForm)]
pub struct SignUpForm {
    uname: String,
    email: String,
    phone_number: String,
    adhaar_number: String,
    pass: String,
    photo: String,
    birthday: String,
}

#[post("/signup", data = "<data>")]
pub async fn p_sign_up(
    ctx: &State<Mutex<HashMap<String, String>>>,
    db: &State<UserRepo>,
    data: Form<SignUpForm>,
) -> (Status, String) {
    dotenv().unwrap_or_else(|_| PathBuf::default());
    let now = Utc::now().timestamp();
    let bytes = general_purpose::STANDARD.decode(&data.photo).unwrap();
    let file_dir = env::var("UPLOAD_PATH").unwrap();
    let file_id = &format!("image-{now}");
    let file_name = &format!("{}/{}", &file_dir, &file_id);
    let new_user = User {
        id: None,
        uname: data.uname.to_owned(),
        pass: sha256sum(&data.pass),
        adhaar_number: data.adhaar_number.to_owned(),
        email: data.email.to_owned(),
        phone_number: data.phone_number.to_owned(),
        birthday: data.birthday.to_owned(),
        photo_id: file_id.to_owned(),
    };
    match db.add_user(new_user).await {
        Some(resp) => {
            // only create file if adding user succeeds
            let mut handle = File::create(file_name).unwrap();
            handle.write_all(&bytes).unwrap();
            resp.unwrap();
            (Status::Created, "Success".to_string())
        }
        None => {
            ctx.lock()
                .unwrap()
                .insert("uname_unavail".to_string(), "true".to_string());
            (Status::Forbidden, "User name taken".to_string())
        }
    }
}
