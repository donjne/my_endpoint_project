#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::http::Status;
use rocket::response::status;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
struct ResponseData {
    slack_name: String,
    current_day: String,
    utc_time: String,
    track: String,
    github_file_url: String,
    github_repo_url: String,
    status_code: u16,
}

#[get("/api?slack_name=DaviesPeeks&track=backend")]

fn get_data(slack_name: String, track: String) -> status::Custom<Json<ResponseData>> {
    let current_day = Utc::now().format("%A").to_string();
    let utc_time = Utc::now().to_string();
    let github_file_url = "https://github.com/donjne/my_endpoint_project/blob/master/src/main.rs".to_string();
    let github_repo_url = "https://github.com/username/repo".to_string();

    let response_data = ResponseData {
        slack_name,
        current_day,
        utc_time,
        track,
        github_file_url,
        github_repo_url,
        status_code: 200,
    };

    status::Custom(Status::Ok, Json(response_data))
}

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    json!({
        "error": "Route not found",
        "path": req.uri().path()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_data])
        .register("/", catchers![not_found])
}
