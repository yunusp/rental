mod models;
mod repo;
use std::{collections::HashMap, sync::Mutex};

use bson::doc;
use models::user_model::User;
use rental::sha256sum;
use rocket::{form::Form, get, launch, post, response::Redirect, routes, uri, FromForm, State};
use rocket_dyn_templates::Template;

use crate::repo::user_repo::UserRepo;

#[get("/")]
async fn index(ctx: &State<Mutex<HashMap<String, String>>>) -> Template {
    let lock = ctx.lock().unwrap().to_owned();
    Template::render("home", lock)
}
#[get("/signin")]
async fn s_sign_in(ctx: &State<Mutex<HashMap<String, String>>>) -> Template {
    let lock = ctx.lock().unwrap().to_owned();
    ctx.lock().unwrap().remove("pass_missmatch");
    ctx.lock().unwrap().remove("uname_unavail");
    Template::render("signin", lock)
}

#[derive(FromForm)]
struct SignInForm {
    uname: String,
    pass: String,
}

#[post("/signin", data = "<data>")]
async fn p_sign_in(
    data: Form<SignInForm>,
    ctx: &State<Mutex<HashMap<String, String>>>,
    db: &State<UserRepo>,
) -> Redirect {
    let hash = sha256sum(&data.pass);
    match db.get_user(&data.uname).await {
        Some(user) => {
            if user.pass == hash {
                ctx.lock()
                    .unwrap()
                    .insert("auth_uname".to_string(), format!("{}", user.uname));
                Redirect::to(uri!("/"))
            } else {
                ctx.lock()
                    .unwrap()
                    .insert("pass_missmatch".to_string(), "true".to_string());
                println!("{:?}", ctx.lock().unwrap());
                Redirect::to(uri!("/signin"))
            }
        }
        None => {
            ctx.lock()
                .unwrap()
                .insert("uname_unavail".to_string(), "true".to_string());
            Redirect::to(uri!("/signin"))
        }
    }
    // Redirect::to(uri!("/"))
}

#[get("/signup")]
async fn s_sign_up(ctx: &State<Mutex<HashMap<String, String>>>) -> Template {
    let lock = ctx.lock().unwrap().to_owned();
    ctx.lock().unwrap().remove("no_pass_match");
    ctx.lock().unwrap().remove("uname_unavail");
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
    ctx: &State<Mutex<HashMap<String, String>>>,
    db: &State<UserRepo>,
    data: Form<SignUpForm>,
) -> Redirect {
    if data.pass != data.pass1 {
        let mut ctx = ctx.lock().unwrap();
        ctx.insert("no_pass_match".to_string(), "a".to_string());
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
        None => {
            ctx.lock()
                .unwrap()
                .insert("uname_unavail".to_string(), "true".to_string());
            return Redirect::to(uri!("/signup"));
        }
    }
    Redirect::to(uri!("/"))
}

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let db = UserRepo::init().await;
    let ctx: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(db)
        .manage(ctx)
        .mount("/", routes![index, s_sign_in,p_sign_in, s_sign_up, p_sign_up])
        .attach(Template::fairing())
}
