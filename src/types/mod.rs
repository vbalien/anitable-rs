use num_enum::TryFromPrimitive;

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
