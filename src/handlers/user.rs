use self::models::*;
use self::schema::users::dsl::*;
use alloy::*;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use rocket::{
    http::Status,
    serde::json::{json, Json, Value},
};
use serde::Deserialize;

use middleware::auth::UserClaim;

// Credentials
#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

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
#[post("/auth/register", format = "json", data = "<new_user>")]
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

#[post("/auth/login", format = "json", data = "<credentials>")]
pub fn auth(credentials: Json<Credentials>) -> Result<Json<Value>, Status> {
    // Verify the username
    let connection: &mut PgConnection = &mut connect_db();
    let results = users
        .filter(username.eq(&credentials.username))
        .select(User::as_select())
        .load::<User>(connection)
        .expect("Error loading users");
    // Return an error if the username is not found
    if results.is_empty() {
        return Err(Status::Unauthorized);
    }

    let user = results.into_iter().next().expect("User not found");

    // Create the token
    let user_claim = UserClaim {
        id: user.id.to_string(),
    };

    let token = UserClaim::sign(user_claim);

    // Verify the password
    match bcrypt::verify(&credentials.password, &user.password) {
        Ok(true) => Ok(Json(json!({
           "access_token": token
        }))),
        Ok(false) => Err(Status::Unauthorized),
        Err(_) => Err(Status::InternalServerError),
    }
}

// get user by id
#[get("/<user_id>")]
pub fn get(user_id: i32) -> Result<Json<Value>, Status> {
    // Get the user from the database
    let connection: &mut PgConnection = &mut connect_db();
    let results = users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .load::<User>(connection)
        .expect("Error loading users");

    // Return an error if the user is not found
    if results.is_empty() {
        return Err(Status::NotFound);
    }

    let user = results.into_iter().next().expect("User not found");

    // Return the user object
    Ok(Json(json!({
        "id": user.id,
        "username": user.username,
        "email": user.email
    })))
}

// User profile
#[get("/me")]
pub fn me(user: UserClaim) -> Json<Value> {
    // Get the user from the database
    let connection: &mut PgConnection = &mut connect_db();
    let results = users
        .filter(id.eq(user.id.parse::<i32>().unwrap()))
        .select(User::as_select())
        .load::<User>(connection)
        .expect("Error loading users");

    let user = results.into_iter().next().expect("User not found");

    // Return the user object
    Json(json!({
        "id": user.id,
        "username": user.username,
        "email": user.email
    }))
}

// Verify token route
#[post("/verify")]
pub fn verify(_user: UserClaim) -> Result<Json<Value>, Status> {
    // if token is valid, return status 200 and json with message
    Ok(Json(json!({
        "message": "Token is valid"
    })))
}
