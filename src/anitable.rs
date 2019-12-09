use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use crate::format::{option_date_format, date_format, datetime_format};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeData {
    #[serde(rename = "a")] pub alive: bool,
    #[serde(rename = "ed", with="option_date_format")] pub end_date: Option<NaiveDate>,
    #[serde(rename = "g")] pub genre: String,
    #[serde(rename = "i")] pub id: i32,
    #[serde(rename = "l")] pub link: String,
    #[serde(rename = "s")] pub subject: String,
    #[serde(rename = "sd", with="date_format")] pub start_date: NaiveDate,
    #[serde(rename = "t")] pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptionData {
    #[serde(rename="a")] pub link: String,
    #[serde(rename="d", with="datetime_format")] pub date: DateTime<Utc>,
    #[serde(rename="n")] pub author: String,
    #[serde(rename="s")] pub episode: String,
}

pub enum Weekday {
    /// Sunday.
    Sun = 0,
    /// Monday.
    Mon = 1,
    /// Tuesday.
    Tue = 2,
    /// Wednesday.
    Wed = 3,
    /// Thursday.
    Thu = 4,
    /// Friday.
    Fri = 5,
    /// Saturday.
    Sat = 6,
    /// Etc
    Etc = 7,
    /// New
    New = 8
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
        postdata.insert("w", week as i32);
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