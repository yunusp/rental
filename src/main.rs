use std::collections::HashMap;

use rocket::{
    get,
    http::{Cookie, CookieJar},
    launch, routes,
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
    for cookie in jar.iter() {
        cs.insert(cookie.name(), cookie.value());
    }
    Template::render("index", context! {names: cs})
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, greet, set_cookie, get_cookies, template],
        )
        .attach(Template::fairing())
    // .mount("/", FileServer::from(relative!("static")))
}
