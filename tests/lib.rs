use anitable::anitable::*;
use mockito::mock;
use chrono::NaiveDate;

#[tokio::test]
async fn test_anitable_list() {
    let _m = mock("POST", "/list")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"
            [
                {
                    "i":4469,
                    "s":"소드 아트 온라인 앨리시제이션 War of Underworld",
                    "t":"0000","g":"판타지 / 액션",
                    "l":"https://sao-alicization.net    ",
                    "a":true,
                    "sd":"20191013",
                    "ed":"00000000"
                },
                {
                    "i":4502,
                    "s":"우리는 공부를 못해 2기",
                    "t":"0030","g":"코미디 / 학원물",
                    "l":"https://boku-ben.com  ",
                    "a":true,"sd":"20191006",
                    "ed":"00000000"
                }
            ]"#)
      .create();

    let client = Anitable::new_with_host(&mockito::server_url());
    let data = client.list(Weekday::Mon).await.unwrap();
    const FORMAT: &'static str = "%Y%m%d";

    assert_eq!(4469, data[0].id);
    assert_eq!(NaiveDate::parse_from_str(&"20191013", FORMAT).unwrap(), data[0].start_date);
}
