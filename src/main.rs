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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}