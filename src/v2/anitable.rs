use crate::types::*;
use crate::v1::format::{datetime_format, option_date_format};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 애니메이션 정보를 담는 자료형
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeItem {
    /// 고유번호
    pub animeNo: i32,
    /// 상태값
    /// `ON`: 반영중
    /// `OFF`: 결방
    pub status: String,
    /// 시각
    /// `%H:%M`
    pub time: Option<String>,
    /// 제목
    pub subject: String,
    /// 장르
    pub genres: String,
    /// 방영 시작일
    /// `%Y-%m-%d`
    pub startDate: Option<String>,
    /// 방영 종료일
    /// `%Y-%m-%d`
    pub endDate: Option<String>,
    /// 홈페이지
    pub website: Option<String>,
    /// 자막참여자 수
    pub captionCount: i32,
}

/// 자막 정보를 담는 자료형
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptionData {
    /// 에피소드
    pub episode: String,
    /// 업데이트 날짜 및 시각
    /// `%Y-%m-%d %H:%M:%S`
    pub updDt: String,
    /// 자막주소
    pub website: Option<String>,
    /// 자막제작자
    pub name: String,
}

/// # 애니편성표 API Wrapper
/// ## Example
/// ```
/// use anitable::v1::*;
/// use anitable::types::*;
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
    /// use anitable::v2::*;
    /// use anitable::types::*;
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
    /// use anitable::v2::*;
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
    ///     use anitable::types::*;
    ///     use anitable::v2::*;
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
    ///     use anitable::v2::*;
    ///     use anitable::types::*;
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
