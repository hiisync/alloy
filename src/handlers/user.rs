use rocket::serde::json::{json, Json, Value};

// Users list
#[get("/")]
pub fn index() -> Json<Value> {
    // TODO: Write code to retrieve a list of users from the database.
    let data = json!([
        {
            "name": "Harry Potter",
            "city": "London"
        },
        {
            "name": "Don Quixote",
            "city": "Madrid"
        },
        {
            "name": "Joan of Arc",
            "city": "Paris"
        },
        {
            "name": "Rosa Park",
            "city": "Alabama"
        }
    ]);

    Json(data)
}
