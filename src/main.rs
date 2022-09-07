use std::collections::HashMap;

use rocket::{
    form::Form,
    get,
    http::{Cookie, CookieJar},
    launch, post,
    response::Redirect,
    routes, uri, FromForm,
};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/hello/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {name}")
}

//set cookies
#[get("/cookie/<name>")]
fn set_cookie(cookie: &CookieJar, name: String) -> &'static str {
    cookie.add(Cookie::new(name, "Hello"));
    "Added cookie"
}
#[get("/cookie")]
fn get_cookies(cookie: &CookieJar) -> String {
    cookie
        .iter()
        .map(|c| format!("{}: {}\n", c.name(), c.value()))
        .collect()
}

//templating
#[get("/temp")]
fn template(jar: &CookieJar) -> Template {
    let mut cs = HashMap::new();
    jar.iter().for_each(|c| {
        cs.insert(c.name(), c.value());
    });
    Template::render("index", context! {names: cs})
}

#[get("/temp/ui")]
fn show_ui(jar: &CookieJar) -> Template {
    let mut cs = HashMap::new();
    jar.iter().for_each(|c| {
        cs.insert(c.name(), c.value());
    });
    Template::render("ui", context! {cs: cs})
}

#[derive(FromForm)]
struct FormData {
    name: String,
    value: String,
}

#[post("/temp/ui", data = "<data>")]
fn add_cookie(jar: &CookieJar, data: Form<FormData>) -> Redirect {
    jar.add(Cookie::new(data.name.to_owned(), data.value.to_owned()));
    Redirect::to(uri!("/temp/ui"))
}

#[get("/temp/ui/clear")]
fn reset_cookies(jar: &CookieJar) -> Redirect {
    for cookie in jar.iter() {
        jar.remove(Cookie::named(cookie.name()).into_owned());
    }
    Redirect::to(uri!("/temp/ui"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                greet,
                set_cookie,
                get_cookies,
                template,
                show_ui,
                add_cookie,
                reset_cookies
            ],
        )
        .attach(Template::fairing())
}
