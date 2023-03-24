use std::{collections::HashMap, sync::Mutex};

use bson::doc;
use rental::sha256sum;
use rocket::{form::Form, get, post, response::Redirect, uri, FromForm, State, Response, http::Status};
use rocket_dyn_templates::Template;

use crate::repo::user_repo::UserRepo;

#[get("/signin")]
pub async fn g_sign_in(ctx: &State<Mutex<HashMap<String, String>>>) -> Template {
    let lock = ctx.lock().unwrap().to_owned();
    ctx.lock().unwrap().remove("pass_missmatch");
    ctx.lock().unwrap().remove("uname_unavail");
    Template::render("signin", lock)
}

#[derive(FromForm, Debug)]
pub struct SignInForm {
    uname: String,
    pass: String,
}

#[post("/signin", data = "<data>")]
pub async fn p_sign_in(
    data: Form<SignInForm>,
    ctx: &State<Mutex<HashMap<String, String>>>,
    db: &State<UserRepo>,
) -> Status {
    let hash = sha256sum(&data.pass);
    println!("{:?}", data);
    Status::Ok
    // match db.get_user(&data.uname).await {
    //     Some(user) => {
    //         if user.pass == hash {
    //             ctx.lock()
    //                 .unwrap()
    //                 .insert("auth_uname".to_string(), format!("{}", user.uname));
    //             Redirect::to(uri!("/"))
    //         } else {
    //             ctx.lock()
    //                 .unwrap()
    //                 .insert("pass_missmatch".to_string(), "true".to_string());
    //             println!("{:?}", ctx.lock().unwrap());
    //             Redirect::to(uri!("/signin"))
    //         }
    //     }
    //     None => {
    //         ctx.lock()
    //             .unwrap()
    //             .insert("uname_unavail".to_string(), "true".to_string());
    //         Redirect::to(uri!("/signin"))
    //     }
    // }
    // Redirect::to(uri!("/"))
}
