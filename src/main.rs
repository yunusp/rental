mod models;
mod repo;
use bson::doc;
use models::note_model::Note;
use repo::note_repo::NoteRepo;
use rocket::{
    form::Form, get, launch, post, response::Redirect, routes, uri, FromForm, State,
};
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
    Template::render("signup", context!{})
}
#[derive(FromForm)]
struct SignUpForm {
    uname: String,
    pass: String,
    pass1: String,
}
#[post("/signup", data="<data>")]
async fn p_sign_up(data: Form<SignUpForm>) -> Redirect {
    
    Redirect::to("/")
}
#[derive(FromForm)]
struct NoteReq {
    text: String,
}

#[post("/", data = "<data>")]
async fn p_index(db: &State<NoteRepo>, data: Form<NoteReq>) -> Redirect {
    if data.text != "" {
        if !db.is_duplicate(&data.text).await {
            db.add_note(Note {
                id: None,
                text: data.text.to_owned(),
            })
            .await
            .unwrap();
        }
    }
    Redirect::to(uri!("/"))
}

#[launch]
async fn rocket() -> _ {
    let db = NoteRepo::init().await;
    rocket::build()
        .manage(db)
        .mount("/", routes![index, p_index, s_sign_in, s_sign_up, p_sign_up])
        .attach(Template::fairing())
}
