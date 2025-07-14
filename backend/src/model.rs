use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: Uuid,

    pub name: String,
    pub email: String,
    pub password_hash: String,

    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}