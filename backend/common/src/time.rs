use chrono::{DateTime, FixedOffset, Offset, TimeZone};

pub trait ToFixedOffset {
    fn to_fixed_offset(&self) -> DateTime<FixedOffset>;
}

impl<Tz: TimeZone> ToFixedOffset for DateTime<Tz> {
    fn to_fixed_offset(&self) -> DateTime<FixedOffset> {
        self.with_timezone(&self.offset().fix())
    }
}
