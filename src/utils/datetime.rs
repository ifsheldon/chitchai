use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DatetimeString(pub String);

impl DatetimeString {
    pub fn get_now() -> Self {
        Self(Local::now().to_rfc3339())
    }
}

impl From<DateTime<Local>> for DatetimeString {
    fn from(dt: DateTime<Local>) -> Self {
        Self(format!("{}", dt.to_rfc3339()))
    }
}

impl From<String> for DatetimeString {
    fn from(s: String) -> Self {
        let parsed: DateTime<Local> = DateTime::parse_from_rfc3339(&s).unwrap().into();
        Self(parsed.to_rfc3339())
    }
}

impl Default for DatetimeString {
    fn default() -> Self {
        Self::get_now()
    }
}