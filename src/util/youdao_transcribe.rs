use std::{fs, path::Path, thread, time::Duration, vec};

use serde::{Deserialize, Serialize};

use crate::app::{
    api::{Api, API},
    impls::transcribe::TranscribeError,
};

use super::json_parse::parse;

async fn transcribe(file_path: String) -> Result<Vec<String>, TranscribeError> {
    let file_name = file_path.split('/').last().unwrap();
    let file_path = format!("./assets/newscasts/transcriptions/{}", file_name);
    let file_path = Path::new(&file_path);

    if file_path.exists() {
        return Ok(parse::<Vec<String>>(
            fs::read_to_string(file_path)
                .map_err(|err| TranscribeError::TranscriptionFileNotFoundError(err.to_string()))?
                .as_str(),
        )
        .map_err(|err| TranscribeError::TranscriptionFileInvalidError(err.to_string()))?);
    }

    let Api { id, secret } = API;

    let size = fs::metadata(file_path)
        .map_err(|err| TranscribeError::TranscriptionFileNotFoundError(err.to_string()))?
        .len();

    todo!();

    Ok(vec![String::new()])
}

#[derive(Serialize, Deserialize, Debug)]
enum YoudaoTranscribeError {
    PrepareError(String),
}

fn sleep(delay_ms: u64) {
    thread::sleep(Duration::from_secs(delay_ms));
}

#[derive(Serialize, Deserialize, Debug)]
struct PrepareResult {
    errorCode: String,
    result: String,
}

async fn prepare(
    (salt, curtime, id, secret, fileName, size): (String, String, String, String, String, u64),
) -> Result<PrepareResult, YoudaoTranscribeError> {
    Ok(reqwest::Client::new()
        .post("http://openapi.youdao.com/api/audio/prepare")
        .form(&[
            ("salt", salt.clone()),
            ("type", String::from("1")),
            ("appKey", id.clone()),
            ("sliceNum", size.div_ceil(1048576_0).to_string()),
            ("name", fileName.clone()),
            ("fileSize", size.to_string()),
            ("curtime", curtime.clone()),
            ("langType", String::from("en")),
            (
                "sign",
                sha256::digest(format!("{}{}{}{}", id, salt, curtime, secret)),
            ),
            ("signType", String::from("v4")),
            ("format", fileName.split(".").last().unwrap().to_string()),
        ])
        .send()
        .await
        .map_err(|err| YoudaoTranscribeError::PrepareError(err.to_string()))?
        .json::<PrepareResult>()
        .await
        .map_err(|err| YoudaoTranscribeError::PrepareError(err.to_string()))?)
}
