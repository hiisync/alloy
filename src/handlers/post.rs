use self::models::*;
use self::schema::posts::dsl::*;
use alloy::*;
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};

// Posts list
#[get("/")]
pub fn list() -> Json<Value> {
    let connection: &mut PgConnection = &mut connect_db();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    let posts_json: Vec<Value> = results
        .iter()
        .map(|post| json!({ "title": &post.title, "body": &post.body }))
        .collect();

    Json(json!(posts_json))
}

// Create a new post
#[post("/", format = "json", data = "<new_post>")]
pub fn create_post(new_post: Json<NewPost>) -> Json<Value> {
    let connection: &mut PgConnection = &mut connect_db();

    // Insert the new post into the database
    let inserted_post: Post = diesel::insert_into(posts)
        .values(&*new_post)
        .get_result(connection)
        .expect("Error creating post");

    Json(json!({
        "id": inserted_post.id,
        "title": inserted_post.title,
        "body": inserted_post.body,
        "published": inserted_post.published,
    }))
}
