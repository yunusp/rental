// use std::{collections::HashMap, sync::Mutex};

// use bson::doc;

// use rocket::{get, State};
// use rocket_dyn_templates::Template;

// #[get("/")]
// pub async fn g_index(ctx: &State<Mutex<HashMap<String, String>>>) -> Template {
//     let lock = ctx.lock().unwrap().to_owned();
//     Template::render("home", lock)
// }
