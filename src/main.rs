mod routes;
mod services;
mod guid;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use crate::routes::todo::AddTodo;
use crate::services::create_mongo_client;

#[launch]
async fn rocket() -> Rocket<Build> {
    let builder = rocket::build();
    let mongodb = &create_mongo_client().await;
        
    builder
        .add_todo(mongodb).await
}