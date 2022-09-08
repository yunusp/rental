mod models;
mod repo;
use bson::doc;
use repo::note_repo::NoteRepo;
use rocket::{form::Form, get, launch, post, response::Redirect, routes, uri, FromForm};
use rocket_dyn_templates::{context, Template};

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
async fn p_sign_up(data: Form<SignUpForm>) -> Redirect {
    if data.pass != data.pass1 {
        //TODO: add to context
        return Redirect::to("/error");
    }
    
    Redirect::to(uri!("/"))
}

#[launch]
async fn rocket() -> _ {
    let db = NoteRepo::init().await;
    rocket::build()
        .manage(db)
        .mount("/", routes![index, s_sign_in, s_sign_up, p_sign_up])
        .attach(Template::fairing())
}
