use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};

use crate::app::impls::{parse_ps, parse_titles, translate};
use crate::util::json_parse;

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
pub async fn today(data: String) -> Result<Json<Vec<Today>>, status::BadRequest<String>> {
    let data = json_parse::parse::<Url>(data.as_str())
        .map_err(|err| status::BadRequest(err.to_string()))?;
    let url = data.url;
    Ok(Json(
        parse_titles::parse_titles(url.as_str())
            .await
            .map_err(|err| status::BadRequest(format!("{:?}", err)))?,
    ))
}

#[get("/api/desc?<id>")]
pub async fn desc(id: String) -> Result<Json<Vec<[String; 2]>>, status::BadRequest<String>> {
    if let Ok(id) = id.parse::<i32>() {
        Ok(Json(
            parse_ps::parse_ps(id)
                .await
                .map_err(|err| status::BadRequest(format!("{:?}", err)))?,
        ))
    } else {
        Ok(Json(vec![[
            String::from("p"),
            String::from(r#"<em>The query param "id" is wrong.</em>"#),
        ]]))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Words {
    words: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransResult {
    result: Vec<String>,
}

#[post("/api/trans", format = "json", data = "<data>")]
pub async fn trans(data: String) -> Result<Json<TransResult>, status::BadRequest<String>> {
    let data = json_parse::parse::<Words>(data.as_str())
        .map_err(|err| status::BadRequest(err.to_string()))?
        .words;
    Ok(Json(TransResult {
        result: translate::translate(data)
            .await
            .map_err(|err| status::BadRequest(format!("{:?}", err)))?,
    }))
}
