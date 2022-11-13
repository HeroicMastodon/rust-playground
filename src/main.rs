mod routes;
mod services;
mod guid;
mod mediator;

use routes::date::get_current_date;
use routes::date::date_plus_month;
use routes::todo::update_task;
use routes::todo::create_task;
use routes::todo::list_tasks;
use routes::todo::get_task_by_id;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use crate::mediator::Services;

#[get("/")]
fn say_hello() -> &'static str {
    "Hello, welcome to the api!"
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let services = Services::init().await;

    rocket::build()
        .manage(services)
        .mount("/api", routes![say_hello, get_current_date, date_plus_month])
        .mount("/api/todo", routes![
            update_task,
            create_task,
            list_tasks,
            get_task_by_id
        ])
}