mod models;
mod repo;
use bson::doc;
use models::user_model::User;
use rental::sha256sum;
use repo::note_repo::NoteRepo;
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
async fn s_sign_up() -> Template {
    Template::render("signup", context! {})
}
#[derive(FromForm)]
struct SignUpForm {
    uname: String,
    pass: String,
    pass1: String,
}
#[post("/signup", data = "<data>")]
async fn p_sign_up(db: &State<UserRepo>, data: Form<SignUpForm>) -> Redirect {
    if data.pass != data.pass1 {
        //TODO: add to context
        return Redirect::to("/errorpass");
    }
    let new_user = User {
        id: None,
        uname: data.uname.to_owned(),
        pass: sha256sum(&data.pass),
    };
    match db.add_user(new_user).await {
    Some(resp) => {resp.unwrap();},
    None => return Redirect::to(uri!("/error")), //TODO: Add to context
}
    // db.add_user(new_user).await.unwrap().unwrap();
    Redirect::to(uri!("/"))
}

#[launch]
async fn rocket() -> _ {
    let db = UserRepo::init().await;
    rocket::build()
        .manage(db)
        .mount("/", routes![index, s_sign_in, s_sign_up, p_sign_up])
        .attach(Template::fairing())
}
