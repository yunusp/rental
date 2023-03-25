use std::{
    collections::HashMap,
    env,
    fs::{write, File},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};
use rental::sha256sum;
use crate::{models::user_model::User, repo::user_repo::UserRepo};
use base64::{engine::general_purpose, Engine as _};
use bson::doc;
use chrono::{self, Utc};
use dotenv::dotenv;
use rocket::{form::Form, get, post, FromForm, State};
use rocket_dyn_templates::Template;
#[get("/signup")]
pub async fn g_sign_up(ctx: &State<Mutex<HashMap<String, String>>>) -> Template {
    let lock = ctx.lock().unwrap().to_owned();
    ctx.lock().unwrap().remove("no_pass_match");
    ctx.lock().unwrap().remove("uname_unavail");
    Template::render("signup", lock)
}
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
) -> String {
    // println!("{data:?}");
    dotenv().unwrap_or_else(|_| PathBuf::default());
    let now = Utc::now().timestamp();
    let bytes = general_purpose::STANDARD.decode(&data.photo).unwrap();
    let file_dir = env::var("UPLOAD_PATH").unwrap();
    let file_id = &format!("image-{now}");
    let file_name = &format!("{}/{}", &file_dir, &file_id);
    let mut handle = File::create(file_name).unwrap();
    handle.write_all(&bytes).unwrap();
    // if data.pass != data.pass1 {
    //     let mut ctx = ctx.lock().unwrap();
    //     ctx.insert("no_pass_match".to_string(), "a".to_string());
    //     return Redirect::to("/signup");
    // }
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
            resp.unwrap();
        }
        None => {
            ctx.lock()
                .unwrap()
                .insert("uname_unavail".to_string(), "true".to_string());
            return "Hello".to_string();
        }
    }
    "Hello".to_string()
    // Redirect::to(uri!("/"))
}
