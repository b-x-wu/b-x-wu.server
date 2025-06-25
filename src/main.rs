#[macro_use] extern crate rocket;

#[get("/hello_world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![hello_world])
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("/static")))
}
