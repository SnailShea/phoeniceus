use time::{
    macros::{date, time},
    OffsetDateTime,
    UtcOffset
};

pub fn rfc868_now() -> i32 {
    let offset = UtcOffset::UTC;
    let base_time = OffsetDateTime::new_in_offset(date!(1900 - 01 - 01), time!(0:00), offset);
    let now = OffsetDateTime::now_utc();
    let seconds = (now - base_time).whole_seconds() as i32;
    seconds
}