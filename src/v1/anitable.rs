use crate::v1::format::{datetime_format, option_date_format};
use chrono::{DateTime, NaiveDate, Utc};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 애니메이션 정보를 담는 자료형
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeData {
    /// 방영중 여부
    #[serde(rename = "a")]
    pub alive: bool,
    /// 종영일
    #[serde(rename = "ed", with = "option_date_format")]
    pub end_date: Option<NaiveDate>,
    /// 장르
    #[serde(rename = "g")]
    pub genre: String,
    /// 고유번호
    #[serde(rename = "i")]
    pub id: i32,
    /// 공식 홈페이지 링크
    #[serde(rename = "l")]
    pub link: String,
    /// 제목
    #[serde(rename = "s")]
    pub subject: String,
    /// 방영 시작일
    #[serde(rename = "sd", with = "option_date_format")]
    pub start_date: Option<NaiveDate>,
    /// 방영시각
    #[serde(rename = "t")]
    pub time: String,
}

/// 자막 정보를 담는 자료형
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptionData {
    /// 다운로드 페이지
    #[serde(rename = "a")]
    pub link: String,
    /// 갱신일
    #[serde(rename = "d", with = "datetime_format")]
    pub date: DateTime<Utc>,
    /// 자막
    #[serde(rename = "n")]
    pub author: String,
    /// 최근 에피소드
    #[serde(rename = "s")]
    pub episode: String,
}

/// 편성표 구분을 위한 `enum`
#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum Tabletype {
    /// 일요일
    Sun = 0u8,
    /// 월요일
    Mon = 1u8,
    /// 화요일
    Tue = 2u8,
    /// 수요일
    Wed = 3u8,
    /// 목요일
    Thu = 4u8,
    /// 금요일
    Fri = 5u8,
    /// 토요일
    Sat = 6u8,
    /// 기타
    Etc = 7u8,
    /// 신작
    New = 8u8,
}

/// # 애니편성표 API Wrapper
/// ## Example
/// ```
/// use anitable::v1::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), reqwest::Error> {
///     let client = Anitable::new();
///     let data = client.list(Tabletype::Sun).await?; // 일요일
///     println!("{:?}", data); // 애니목록 출력
///
///     let data = client.cap(data[0].id).await?; // 애니목록 0번째 자막
///     println!("{:?}", data); // 자막목록 출력
///     Ok(())
/// }
/// ```
pub struct Anitable {
    client: reqwest::Client,
    url: String,
}

impl Anitable {
    /// 테스트를 위한 생성자
    /// # Example
    /// ```
    /// use anitable::v1::*;
    /// use mockito::mock;
    /// let client = Anitable::new_with_host(&mockito::server_url());
    /// ```
    pub fn new_with_host(host: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: String::from(host),
        }
    }

    /// 일반 생성자
    /// # Example
    /// ```
    /// use anitable::v1::*;
    /// let client = Anitable::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            url: String::from("https://www.anissia.net/anitime/"),
        }
    }

    /// 해당하는 타입의 애니메이션 편성목록을 리턴.
    ///
    /// # Arguments
    ///
    /// * `tabletype` - 편성표 구분
    ///
    /// # Example
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() -> Result<(), reqwest::Error> {
    ///     use anitable::v1::*;
    ///     let client = Anitable::new();
    ///     let animations = client.list(Tabletype::Sun).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self, tabletype: Tabletype) -> Result<Vec<AnimeData>, reqwest::Error> {
        let mut postdata = HashMap::new();
        postdata.insert("w", tabletype as u8);
        let animes: Vec<AnimeData> = self
            .client
            .post(&format!("{}/list", self.url))
            .form(&postdata)
            .send()
            .await?
            .json()
            .await?;
        Ok(animes)
    }

    /// 입력받은 애니메이션의 자막 목록을 리턴.
    ///
    /// # Arguments
    ///
    /// * `anime_id` - 애니메이션의 고유번호
    ///
    /// # Example
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() -> Result<(), reqwest::Error> {
    ///     use anitable::v1::*;
    ///     let client = Anitable::new();
    ///     let animations = client.list(Tabletype::Sun).await?;
    ///     let captions = client.cap(animations.get(0).unwrap().id).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn cap(&self, anime_id: i32) -> Result<Vec<CaptionData>, reqwest::Error> {
        let mut postdata = HashMap::new();
        postdata.insert("i", anime_id);
        let caps: Vec<CaptionData> = self
            .client
            .post(&format!("{}/cap", self.url))
            .form(&postdata)
            .send()
            .await?
            .json()
            .await?;
        Ok(caps)
    }
}
