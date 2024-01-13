#[macro_use]
extern crate rocket;

mod handlers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handlers::user::index])
}
