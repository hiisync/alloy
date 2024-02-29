#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket_cors::{AllowedOrigins, CorsOptions};

pub mod handlers;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&[
            "http://localhost:3000",
            "http://127.0.0.1:3000",
        ]))
        .to_cors()
        .expect("Error creating CORS fairing");

    rocket::build()
        .attach(cors)
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
                handlers::user::get,
                handlers::user::verify,
            ],
        )
}
