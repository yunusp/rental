use rocket::{fs::relative, fs::FileServer, get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/hello/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {name}")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, greet])
        .mount("/", FileServer::from(relative!("static")))
}
