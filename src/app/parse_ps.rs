use base64::Engine;
use scraper::Html;
use serde::{Deserialize, Serialize};

use crate::my_trait::query::Query;

#[derive(Serialize, Deserialize, Debug)]
pub enum ParsePsError {
    ResponseError(String),
    BodyError(String),
}

pub async fn parse_ps(id: i32) -> Result<Vec<[String; 2]>, ParsePsError> {
    let html = reqwest::get(format!("https://www.voanews.com/a/{id}.html"))
        .await
        .map_err(|err| ParsePsError::ResponseError(err.to_string()))?
        .text()
        .await
        .map_err(|err| ParsePsError::BodyError(err.to_string()))?;
    let document = Html::parse_document(&html);

    let title = document.query_selector("h1").inner_html();
    let title = title.trim();
    if title == "VOA Newscasts" {
        Ok(vec![
            [String::from("h1"), String::from(title)],
            [
                String::from("div"),
                document
                    .query_selector("time")
                    .inner_html()
                    .trim()
                    .to_string(),
            ],
            [
                String::from("video"),
                format!(
                    r#"<source src="https://voanews.jtp0415.top/api/transcribe/audio?url=${}" type="audio/mpeg">"#,
                    base64::engine::general_purpose::STANDARD.encode(
                        document
                            .query_selector(".c-mmp")
                            .query_selector("a")
                            .attr("href")
                            .unwrap()
                    )
                ),
            ],
        ])
    } else {
        Ok({
            let mut v = vec![
                [String::from("h1"), String::from(title)],
                [
                    String::from("div"),
                    document
                        .query_selector("time")
                        .inner_html()
                        .trim()
                        .to_string(),
                ],
            ];
            v.append(
                document
                    .query_selector("#article-content")
                    .query_selector_all("p")
                    .iter()
                    .map(|item| [item.value().name().to_string(), item.inner_html()])
                    .collect::<Vec<[String; 2]>>()
                    .as_mut(),
            );
            v
        })
    }
}
