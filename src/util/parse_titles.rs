use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::app::main::Today;

pub enum ParseTitlesError {
    ResponseError(String),
    BodyError(String),
    TooManyMediaBlockWrap,
    TooManyA,
    ANoTitle,
    ANoId,
}

pub async fn parse_titles(url: &str) -> Result<Vec<Today>, ParseTitlesError> {
    let regex =
        Regex::new(r"^https:\/\/www\.voanews\.com\/(?:[a-z-]+|p\/\d+\.html|z\/\d+)$").unwrap();
    if regex.is_match(url) {
        return Ok(vec![Today {
            title: r#"Found "Cross Site Scripting (XSS)", stop parsing."#.to_string(),
            id: String::new(),
        }]);
    }

    let html = reqwest::get("")
        .await
        .map_err(|err| ParseTitlesError::ResponseError(err.to_string()))?
        .text()
        .await
        .map_err(|err| ParseTitlesError::BodyError(err.to_string()))?;
    let document = Html::parse_document(&html);
    if url.starts_with("https://www.voanews.com/z/")
        || url == "https://www.voanews.com/voa1-the-hits"
    {
        Ok(Ok(document
            .select(&Selector::parse(".media-block-wrap").unwrap())
            .into_iter()
            .map(|item| item)
            .collect::<Vec<ElementRef>>())
        .and_then::<Vec<ElementRef>, _>(|o| {
            if o.len() != 1 {
                Err(ParseTitlesError::TooManyMediaBlockWrap)
            } else {
                Ok(o)
            }
        })?
        // media_block_wrap
        .into_iter()
        .collect::<Vec<ElementRef>>()[0]
            .select(&Selector::parse(".media-block").unwrap())
            // media_block
            .into_iter()
            .filter(|item| {
                item.select(&Selector::parse(".ico-video").unwrap())
                    .into_iter()
                    .count()
                    == 0
            })
            .map(|item| -> Result<Today, ParseTitlesError> {
                let a = Ok(item
                    .select(&Selector::parse("a").unwrap())
                    .into_iter()
                    .map(|item| item)
                    .collect::<Vec<ElementRef>>())
                .and_then::<Vec<ElementRef>, _>(|a| {
                    if a.into_iter().count() != 1 {
                        Err(ParseTitlesError::TooManyA)
                    } else {
                        Ok(a)
                    }
                })?[0];
                let title = a.attr("title").ok_or(ParseTitlesError::ANoTitle)?;
                let id = a.attr("href").ok_or(ParseTitlesError::ANoId)?;
                Ok(Today {
                    title: String::from(title),
                    id: String::from(id),
                })
            })
            .collect::<Result<Vec<Today>, ParseTitlesError>>()?)
    } else {
        Ok()
    }
}
