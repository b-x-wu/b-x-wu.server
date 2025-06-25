#[macro_use] extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![ping])
        .mount("/", rocket::fs::FileServer::from("/app/static"))
}
