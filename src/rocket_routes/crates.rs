use crate::repositories::CrateRepository;
use rocket::{ http::Status, response::status::{Custom, NoContent}, serde::json::{json, Json}};
use rocket_db_pools::Connection;
use rocket::serde::json::Value;


use crate::models::{NewCrate, Crate};
use crate::rocket_routes::DbConn;

#[rocket::get("/creates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 1000).await
        .map(|crates| json!(crates))
        .map_err(|e| {
            rocket::error!("Error: {}", e);
            Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
        })
}

#[rocket::get("/creates/<id>")]
pub async fn get_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id).await
        .map(|a_crate| json!(a_crate))
        .map_err(|e| {
            rocket::error!("Error: {}", e);
            Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
        })
}

#[rocket::post("/creates", format="json", data = "<new_crates>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crates: Json<NewCrate>) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crates.into_inner()).await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|e| {
            rocket::error!("Error: {}", e);
            Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
        })
}

#[rocket::put("/creates/<id>", format="json", data = "<creates>")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, creates: Json<Crate>) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, creates.into_inner()).await
        .map(|crates|  json!(crates))
        .map_err(|e| {
            rocket::error!("Error: {}", e);
            Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
        })
}

#[rocket::delete("/creates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| {
            rocket::error!("Error: {}", e);
            Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
        })
}
