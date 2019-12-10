use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use crate::format::{option_date_format, datetime_format};
use num_enum::TryFromPrimitive;


#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeData {
    #[serde(rename = "a")] pub alive: bool,
    #[serde(rename = "ed", with="option_date_format")] pub end_date: Option<NaiveDate>,
    #[serde(rename = "g")] pub genre: String,
    #[serde(rename = "i")] pub id: i32,
    #[serde(rename = "l")] pub link: String,
    #[serde(rename = "s")] pub subject: String,
    #[serde(rename = "sd", with="option_date_format")] pub start_date: Option<NaiveDate>,
    #[serde(rename = "t")] pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptionData {
    #[serde(rename="a")] pub link: String,
    #[serde(rename="d", with="datetime_format")] pub date: DateTime<Utc>,
    #[serde(rename="n")] pub author: String,
    #[serde(rename="s")] pub episode: String,
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum Weekday {
    /// Sunday.
    Sun = 0u8,
    /// Monday.
    Mon = 1u8,
    /// Tuesday.
    Tue = 2u8,
    /// Wednesday.
    Wed = 3u8,
    /// Thursday.
    Thu = 4u8,
    /// Friday.
    Fri = 5u8,
    /// Saturday.
    Sat = 6u8,
    /// Etc
    Etc = 7u8,
    /// New
    New = 8u8
}

pub struct Anitable {
    client: reqwest::Client,
    url: String,
}

impl Anitable {
    pub fn new_with_host(host: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: String::from(host),
        }
    }

    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            url: String::from("https://www.anissia.net/anitime/"),
        }
    }

    pub async fn list(&self, week: Weekday) -> Result<Vec<AnimeData>, reqwest::Error> {
        let mut postdata = HashMap::new();
        postdata.insert("w", week as u8);
        let animes: Vec<AnimeData> = self.client
            .post(&format!("{}/list", self.url))
            .form(&postdata)
            .send()
            .await?
            .json()
            .await?;
        Ok(animes)
    }

    pub async fn cap(&self, anime_id: i32) -> Result<Vec<CaptionData>, reqwest::Error> {
        let mut postdata = HashMap::new();
        postdata.insert("i", anime_id);
        let caps: Vec<CaptionData> = self.client
            .post(&format!("{}/cap", self.url))
            .form(&postdata)
            .send()
            .await?
            .json()
            .await?;
        Ok(caps)
    }
}