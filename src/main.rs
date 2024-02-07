#[macro_use]
extern crate rocket;

pub mod handlers;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/posts",
            routes![handlers::post::list, handlers::post::create_post],
        )
        .mount(
            "/users",
            routes![handlers::user::list, handlers::user::create_user],
        )
}
