#[cfg(feature = "ssr")]
use actix_web::cookie::time::Duration;
#[cfg(feature = "ssr")]
use chrono::{DateTime, Utc};

#[cfg(feature = "ssr")]
fn i128_into_i64(n: i128) -> Option<i64> {
    if n > i64::MAX as i128 {
        None
    } else {
        Some(n as i64)
    }
}

#[cfg(feature = "ssr")]
pub(crate) fn add_duration_to_current(duration: &Duration) -> Option<DateTime<Utc>> {
    let offset = match i128_into_i64(duration.whole_milliseconds()) {
        Some(o) => o,
        None => {
            return None;
        }
    };
    
    let current_timestamp = Utc::now().timestamp_millis();
    let updated_timestamp = current_timestamp + offset;

    let naive_date = DateTime::<Utc>::from_timestamp_millis(updated_timestamp)?;

    Some(naive_date)
}
