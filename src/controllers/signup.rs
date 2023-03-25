use std::{collections::HashMap, sync::Mutex};

use bson::doc;
use rental::sha256sum;
use rocket::{form::Form, get, post, response::Redirect, uri, FromForm, State};
use rocket_dyn_templates::Template;

use crate::{models::user_model::User, repo::user_repo::UserRepo};

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
    pass1: String,
    photo: String,
    birthday: String,
}

#[post("/signup", data = "<data>")]
pub async fn p_sign_up(
    ctx: &State<Mutex<HashMap<String, String>>>,
    db: &State<UserRepo>,
    data: Form<SignUpForm>,
) -> String {
    println!("{data:?}");
    "Hello".to_string()
    // if data.pass != data.pass1 {
    //     let mut ctx = ctx.lock().unwrap();
    //     ctx.insert("no_pass_match".to_string(), "a".to_string());
    //     return Redirect::to("/signup");
    // }
    // let new_user = User {
    //     id: None,
    //     uname: data.uname.to_owned(),
    //     pass: sha256sum(&data.pass),
    // };
    // match db.add_user(new_user).await {
    //     Some(resp) => {
    //         resp.unwrap();
    //     }
    //     None => {
    //         ctx.lock()
    //             .unwrap()
    //             .insert("uname_unavail".to_string(), "true".to_string());
    //         return Redirect::to(uri!("/signup"));
    //     }
    // }
    // Redirect::to(uri!("/"))
}
