mod models;
mod repo;
use models::note_model::Note;
use repo::note_repo::NoteRepo;
use rocket::{form::Form, get, launch, post, response::Redirect, routes, uri, FromForm, State};
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn index(db: &State<NoteRepo>) -> Template {
    let cs: Vec<String> = db
        .get_notes()
        .await
        .unwrap()
        .iter()
        .map(move |note| note.text.to_owned())
        .collect();
    Template::render("ui", context! {cs})
}

#[derive(FromForm)]
struct NoteReq {
    text: String,
}

#[post("/", data = "<data>")]
async fn p_index(db: &State<NoteRepo>, data: Form<NoteReq>) -> Redirect {
    if data.text != "".to_string() {
        db.add_note(Note {
            id: None,
            text: data.text.to_owned(),
        })
        .await
        .unwrap();
    }
    Redirect::to(uri!("/"))
}

#[launch]
async fn rocket() -> _ {
    let db = NoteRepo::init().await;
    rocket::build()
        .manage(db)
        .mount("/", routes![index, p_index])
        .attach(Template::fairing())
}
