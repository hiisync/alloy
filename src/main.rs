#[macro_use]
extern crate rocket;
use dotenvy::dotenv;

pub mod handlers;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount(
            "/posts",
            routes![handlers::post::list, handlers::post::create_post],
        )
        .mount(
            "/users",
            routes![
                handlers::user::list,
                handlers::user::create_user,
                handlers::user::auth,
                handlers::user::me,
            ],
        )
}
