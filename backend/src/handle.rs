use serde::{Deserialize, Serialize};
use validator::Validate;

// Query parameters for pagination
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Path parameters for fetching a single resource by ID
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

// Schema for user registration with validation
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RegisterUserSchema {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

// Schema for user login with validation
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserSchema {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

// Schema for updating user profile
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UpdateUserSchema {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
}