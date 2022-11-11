use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

use crate::services;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32
}

#[get("/date/get-current-date")]
pub fn get_current_date() -> Json<Date> {
    Json(services::date::get_current_date())
}

#[post("/date/date-plus-month", format = "json", data = "<date>")]
pub fn date_plus_month(date: Json<Date>) -> Json<Date> {
    Json(services::date::date_plus_month(date))
}
