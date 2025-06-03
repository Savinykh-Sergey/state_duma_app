pub type ResultWithStringError<T> = Result<T, String>;

pub mod custom_serde {
    use chrono::NaiveDate;
    use serde::{Serializer, Deserializer, Deserialize};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = date.and_hms_opt(0, 0, 0).expect("Invalid date");
        serializer.serialize_i64(datetime.and_utc().timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;
        let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?;
        Ok(datetime.date_naive())
    }
}
