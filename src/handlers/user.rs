use self::models::*;
use self::schema::users::dsl::*;
use alloy::*;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};

// Users list
#[get("/")]
pub fn list() -> Json<Value> {
    let connection: &mut PgConnection = &mut connect_db();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    let users_json: Vec<Value> = results
        .iter()
        .map(|user: &User| json!({ "id": &user.id, "username": &user.username, "email": &user.email }))
        .collect();

    Json(json!(users_json))
}

// Create a new user
#[post("/store", format = "json", data = "<new_user>")]
pub fn create_user(mut new_user: Json<NewUser>) -> Json<Value> {
    let connection: &mut PgConnection = &mut connect_db();

    // Converting a password to a hash
    new_user.password = hash(&new_user.password, DEFAULT_COST).unwrap();

    // Insert the new user into the database
    let inserted_user: User = diesel::insert_into(users)
        .values(&*new_user)
        .get_result(connection)
        .expect("Error creating user");

    Json(json!({
        "id": inserted_user.id,
        "username": inserted_user.username,
        "email": inserted_user.email,
        "created_at": inserted_user.created_at,
        "updated_at": inserted_user.updated_at
    }))
}
