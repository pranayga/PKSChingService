#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/pandu")]
fn pandu() -> &'static str {
    "Hello Pandu!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, pandu])
}
