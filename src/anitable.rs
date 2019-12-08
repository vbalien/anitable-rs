use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use crate::format::{option_date_format, date_format, datetime_format};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeData {
    #[serde(rename = "a")] alive: bool,
    #[serde(rename = "ed", with="option_date_format")] end_date: Option<NaiveDate>,
    #[serde(rename = "g")] genre: String,
    #[serde(rename = "i")] id: i32,
    #[serde(rename = "l")] link: String,
    #[serde(rename = "s")] subject: String,
    #[serde(rename = "sd", with="date_format")] start_date: NaiveDate,
    #[serde(rename = "t")] time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptionData {
    #[serde(rename="a")] link: String,
    #[serde(rename="d", with="datetime_format")] date: DateTime<Utc>,
    #[serde(rename="n")] author: String,
    #[serde(rename="s")] episode: String,
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
    client: reqwest::Client
}

impl Anitable {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new()
        }
    }

    pub async fn list(&self, week: Weekday) -> Result<Vec<AnimeData>, reqwest::Error> {
        let mut postdata = HashMap::new();
        postdata.insert("w", week as i32);
        let animes: Vec<AnimeData> = self.client
            .post("https://www.anissia.net/anitime/list")
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
            .post("https://www.anissia.net/anitime/cap")
            .form(&postdata)
            .send()
            .await?
            .json()
            .await?;
        Ok(caps)
    }
}