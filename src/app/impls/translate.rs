use std::time::{SystemTime, UNIX_EPOCH};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::api::{Api, API};

#[derive(Serialize, Deserialize, Debug)]
pub enum TranslateError {
    ResponseError(String),
    ParseBodyError(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslateResult {
    isWord: bool,

    query: String,
    translation: Vec<String>,

    returnPhrase: Vec<String>,
    basic: T1,
}

#[derive(Serialize, Deserialize, Debug)]
struct T1 {
    explains: Vec<String>,
}

pub async fn translate(q: String) -> Result<Vec<String>, TranslateError> {
    let Api { id, secret } = API;
    let salt = Uuid::new_v4().to_string();
    let curtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let result = reqwest::Client::new()
        .post("https://openapi.youdao.com/api")
        .form(&[
            ("q", q.clone()),
            ("from", String::from("en")),
            ("to", String::from("zh-CHS")),
            ("appKey", String::from(id)),
            ("salt", salt.clone()),
            (
                "sign",
                sha256::digest(format!(
                    "{}{}{}{}{}",
                    id,
                    if q.len() <= 20 {
                        q
                    } else {
                        format!(
                            "{}{}{}",
                            q.split_at(10).0,
                            q.len(),
                            q.split_at(q.len() - 10).1,
                        )
                    },
                    salt,
                    curtime,
                    secret,
                )),
            ),
            ("signType", String::from("v3")),
            ("curtime", curtime.to_string()),
        ])
        .send()
        .await
        .map_err(|err| TranslateError::ResponseError(err.to_string()))?
        .json::<TranslateResult>()
        .await
        .map_err(|err| TranslateError::ParseBodyError(err.to_string()))?;

    match result.isWord {
        true => Ok(vec![
            result.returnPhrase.join(" / "),
            result.translation.join(" / "),
            result.basic.explains.join("\n"),
        ]),
        false => Ok(vec![result.query, result.translation.join(" / ")]),
    }
}
