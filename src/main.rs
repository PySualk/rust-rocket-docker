use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    format!("Hello, world from {}! I was compiled on {}.", CURRENT_PLATFORM, COMPILED_ON)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
