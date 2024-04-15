use chrono::NaiveDateTime;
use diesel::prelude::* ;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize ,Deserialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable,Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable,Deserialize, Serialize)]
#[diesel(table_name = crates )]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Queryable,Debug, Serialize, Deserialize, AsChangeset,Identifiable)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable,Deserialize, Serialize)]
#[diesel(table_name = users )]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize,Debug, Deserialize,Identifiable, AsChangeset)]
pub struct Role {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub code: String,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable,Deserialize, Serialize)]
#[diesel(table_name = roles )]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
