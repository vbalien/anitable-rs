use ::anitable::anitable::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Anitable::new();
    let data = client.list(Weekday::Mon).await?;
    println!("{:?}", data);
    Ok(())
}
