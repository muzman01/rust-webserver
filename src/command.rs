use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{models::NewUser, repositories::{RoleRepository, UserRepository}};

async fn load_db_connection() -> AsyncPgConnection{
    // Load database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url).await.expect("Error connecting to database")
}

pub async fn create_users(username:String, password:String, role_codes:Vec<String>){
    // Create a new user
    let mut c = load_db_connection().await;
    let new_user = NewUser{username, password};
    let user = UserRepository::create(&mut c, new_user, role_codes).await.unwrap();
    println!("user oluştu {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("atanan roller {:?}", roles);
}

pub async fn list_users(){
    let mut c = load_db_connection().await;
    let users = UserRepository::find_with_roles(&mut c).await.unwrap();
    println!("users {:?}", users);
    // List existing users

}

pub async fn delete_users(id:i32){
    // let mut c = load_db_connection().await;
    println!("Delete by user ID {}", id);


    // Delete by user ID

}