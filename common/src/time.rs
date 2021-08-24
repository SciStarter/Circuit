use chrono::{DateTime, FixedOffset, Local, Utc};

pub trait ToFixedOffset {
    fn to_fixed_offset(&self) -> DateTime<FixedOffset>;
}

// Need the specialization feature to arrive before we can get away
// with doing this and also have implementations for e.g. Utc

// impl<Tz> ToFixedOffset for DateTime<Tz>
// where
//     Tz: TimeZone<Offset = FixedOffset>,
// {
//     fn to_fixed_offset(&self) -> DateTime<FixedOffset> {
//         self.with_timezone(self.offset())
//     }
// }

impl ToFixedOffset for DateTime<Local> {
    fn to_fixed_offset(&self) -> DateTime<FixedOffset> {
        self.with_timezone(self.offset())
    }
}

impl ToFixedOffset for DateTime<Utc> {
    fn to_fixed_offset(&self) -> DateTime<FixedOffset> {
        self.with_timezone(&FixedOffset::west(0))
    }
}

impl ToFixedOffset for DateTime<FixedOffset> {
    fn to_fixed_offset(&self) -> DateTime<FixedOffset> {
        self.clone()
    }
}
