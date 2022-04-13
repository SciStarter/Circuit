use chrono::{NaiveDate, Offset, TimeZone};
use chrono_tz::{Tz, TZ_VARIANTS};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid local time")]
    InvalidLocalTime,
    #[error("invalid timezone name")]
    InvalidTimezoneName,
}

#[derive(Serialize, Debug)]
pub struct Zone {
    pub offset: String,
    pub offset_minutes: i32,
}

pub fn timezone<S>(name: S, at: NaiveDate) -> Result<Zone, Error>
where
    S: AsRef<str>,
{
    let tz: Tz = name
        .as_ref()
        .parse()
        .map_err(|_| Error::InvalidTimezoneName)?;
    let offset = tz
        .offset_from_local_date(&at)
        .latest()
        .ok_or_else(|| Error::InvalidLocalTime)?;

    let offset_minutes = offset.fix().local_minus_utc() / 60;
    let offset_hh = offset_minutes.abs() / 60;
    let offset_mm = offset_minutes.abs() % 60;

    Ok(Zone {
        offset: format!(
            "{}{:02}:{:02}",
            if offset_minutes < 0 { "-" } else { "+" },
            offset_hh,
            offset_mm
        ),
        offset_minutes,
    })
}

pub fn timezones() -> Vec<&'static str> {
    TZ_VARIANTS.iter().map(|tz| tz.name()).collect()
}
