# Anitable
[![Crates.io](https://img.shields.io/crates/v/anitable)](https://crates.io/crates/anitable)
[![documentation](https://docs.rs/anitable/badge.svg)](https://docs.rs/anitable)
![Crates.io](https://img.shields.io/crates/l/anitable)

애니시아 애니편성표 API client library

### 사용예제
```rust
use anitable::*;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let client = Anitable::new();
    let data = client.list(Weekday::Sun).await?; // 일요일
    println!("{:?}", data); // 애니목록 출력

    let data = client.cap(data[0].id).await?; // 애니목록 0번째 자막
    println!("{:?}", data); // 자막목록 출력
    Ok(())
}
```

## License
MIT
