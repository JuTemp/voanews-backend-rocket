use std::{
    io::{self, Cursor},
    path::Path,
    sync::Mutex,
};

use regex::Regex;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum DownloadError {
    ResponseError(String),
    DownloadBodyError(String),
    FileCreateError(String),
    FileSavingError(String),

    LockDownloadError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TranscribeError {
    DownloadError(DownloadError),
    AudioUrlError(String),
    TranscriptionUrlError,
    TranscriptionFileNotFoundError(String),
    TranscriptionFileInvalidError(String),
}

async fn download(lock: Mutex<()>, url: String) -> Result<String, DownloadError> {
    let file_name = url.split("/").last().unwrap();
    let file_path = format!("./assets/newscasts/audios/{}", file_name);
    let file_path = Path::new(&file_path);

    let _l = lock
        .lock()
        .map_err(|err| DownloadError::LockDownloadError(err.to_string()))?;

    if !file_path.exists() {
        let response = reqwest::get(url)
            .await
            .map_err(|err| DownloadError::ResponseError(err.to_string()))?;
        io::copy(
            &mut Cursor::new(
                response
                    .bytes()
                    .await
                    .map_err(|err| DownloadError::DownloadBodyError(err.to_string()))?,
            ),
            &mut std::fs::File::create(file_path)
                .map_err(|err| DownloadError::FileCreateError(err.to_string()))?,
        )
        .map_err(|err| DownloadError::FileSavingError(err.to_string()))?;
    }

    Ok(file_path.display().to_string())
}

pub async fn get_audio_file_path(lock: Mutex<()>, url: String) -> Result<String, TranscribeError> {
    if !Regex::new(r#"^https:\/\/av\.voanews\.com\/clips\/VEN\/\d{4}\/\d{2}\/\d{2}\/\d{8}-\d{6}-VEN119-program\.mp3$"#).unwrap().is_match(&url) {
        return Err(TranscribeError::AudioUrlError(String::from("Invalid Audio Url.")));
    }

    Ok(download(lock, url)
        .await
        .map_err(|err| TranscribeError::DownloadError(err))?)
}

pub async fn get_transcription_string_array(
    lock: Mutex<()>,
    url: String,
) -> Result<Vec<String>, TranscribeError> {
    if !Regex::new(r#"^https:\/\/av\.voanews\.com\/clips\/VEN\/\d{4}\/\d{2}\/\d{2}\/\d{8}-\d{6}-VEN119-program\.mp3$"#).unwrap().is_match(&url) {
        return Err(TranscribeError::TranscriptionUrlError);
    }

    Ok(transcribe(
        download(lock, url)
            .await
            .map_err(|err| TranscribeError::DownloadError(err))?,
    ))
}

