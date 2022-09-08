mod models;
mod repo;
use std::{collections::HashMap, sync::Mutex};

use bson::doc;
use models::user_model::User;
use rental::sha256sum;
use rocket::{form::Form, get, launch, post, response::Redirect, routes, uri, FromForm, State};
use rocket_dyn_templates::{context, Template};

use crate::repo::user_repo::UserRepo;

#[get("/")]
async fn index() -> Template {
    #[allow(non_snake_case)]
    Template::render("home", context! {loggedIn: false})
}
#[get("/signin")]
async fn s_sign_in() -> Template {
    Template::render("signin", context! {})
}
#[get("/signup")]
async fn s_sign_up(ctx: &State<Mutex<HashMap<String, bool>>>) -> Template {
    let lock = ctx.lock().unwrap().to_owned();
    Template::render("signup", lock)
}
#[derive(FromForm)]
struct SignUpForm {
    uname: String,
    pass: String,
    pass1: String,
}
#[post("/signup", data = "<data>")]
async fn p_sign_up(
    ctx: &State<Mutex<HashMap<String, bool>>>,
    db: &State<UserRepo>,
    data: Form<SignUpForm>,
) -> Redirect {
    if data.pass != data.pass1 {
        //TODO: add to context
        let mut ctx = ctx.lock().unwrap();
        ctx.insert("no_pass_match".to_string(), true);
        println!("{:?}", ctx);
        return Redirect::to("/signup");
    }
    let new_user = User {
        id: None,
        uname: data.uname.to_owned(),
        pass: sha256sum(&data.pass),
    };
    match db.add_user(new_user).await {
        Some(resp) => {
            resp.unwrap();
        }
        None => return Redirect::to(uri!("/error")), //TODO: Add to context
    }
    Redirect::to(uri!("/"))
}

#[launch]
async fn rocket() -> _ {
    let db = UserRepo::init().await;
    let ctx: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(db)
        .manage(ctx)
        .mount("/", routes![index, s_sign_in, s_sign_up, p_sign_up])
        .attach(Template::fairing())
}
