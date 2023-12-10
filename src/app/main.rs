use rocket::{post};
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::util::{json_parse, parse_titles};

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Today {
    pub title: String,
    pub id: String,
}

#[post("/api/today", format = "json", data = "<data>")]
async fn today(data: String) -> Result<Json<Today>, status::BadRequest<String>> {
    let data = json_parse::parse::<Url>(data.as_str()).map_err(|err| status::BadRequest(err.to_string()))?;
    let url = data.url;
    Ok(Json(parse_titles::parse_titles(url.as_str()).await))
}
