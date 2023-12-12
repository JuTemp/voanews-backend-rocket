use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::app::main::Today;
use crate::my_trait::query::Query;

#[derive(Serialize, Deserialize, Debug)]
pub enum ParseTitlesError {
    ResponseError(String),
    BodyError(String),
    TooManyMediaBlockWrap,
    TooManyA,
    ANoTitle,
    ANoId,
}

pub async fn parse_titles(url: &str) -> Result<Vec<Today>, ParseTitlesError> {
    if !Regex::new(r"^https:\/\/www\.voanews\.com\/(?:[a-z-]+|p\/\d+\.html|z\/\d+)$")
        .unwrap()
        .is_match(url)
    {
        return Ok(vec![Today {
            title: r#"Found "Cross Site Scripting (XSS)", stop parsing."#.to_string(),
            id: String::new(),
        }]);
    }

    let html = reqwest::get(url)
        .await
        .map_err(|err| ParseTitlesError::ResponseError(err.to_string()))?
        .text()
        .await
        .map_err(|err| ParseTitlesError::BodyError(err.to_string()))?;
    let document = Html::parse_document(&html);
    if url.starts_with("https://www.voanews.com/z/")
        || url == "https://www.voanews.com/voa1-the-hits"
    {
        Ok(document
            .query_selector(".media-block-wrap")
            .query_selector_all(".media-block")
            .iter()
            .filter(|item| {
                item.select(&Selector::parse(".ico-video").unwrap())
                    .into_iter()
                    .count()
                    == 0
            })
            .map(|item| -> Result<Today, ParseTitlesError> {
                let a = item.query_selector("a");
                Ok(Today {
                    title: a
                        .attr("title")
                        .ok_or(ParseTitlesError::ANoTitle)?.trim()
                        .to_string(),
                    id: Regex::new(r"^.*?\/(\d+)\.html$")
                        .unwrap()
                        .captures(a.attr("href").ok_or(ParseTitlesError::ANoId)?)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_string(),
                })
            })
            .collect::<Result<Vec<Today>, ParseTitlesError>>()?)
    } else {
        Ok(document
            .query_selector_all(".media-block").iter()
            .filter(|item| {
                item.select(&Selector::parse(".ico-video").unwrap())
                    .into_iter()
                    .count()
                    == 0
            })
            // media_block
            .map(|item| -> Result<Today, ParseTitlesError> {
                let a = item.query_selector("a");
                Ok(Today {
                    title: a
                        .attr("title")
                        .ok_or(ParseTitlesError::ANoTitle)?.trim()
                        .to_string(),
                    id: Regex::new(r"^.*?\/(\d+)\.html$")
                        .unwrap()
                        .captures(a.attr("href").ok_or(ParseTitlesError::ANoId)?)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_string(),
                })
            })
            .collect::<Result<Vec<Today>, ParseTitlesError>>()?)
    }
}
