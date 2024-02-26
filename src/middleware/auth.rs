use once_cell::sync::Lazy;
use rocket_jwt::jwt;
use std::env;

static SECRET_KEY: Lazy<String> =
    Lazy::new(|| env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set"));

#[jwt(SECRET_KEY, exp = 2592000 /* 30 days */ )]
pub struct UserClaim {
    pub id: String,
}
