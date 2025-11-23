use serde::{Deserialize, Serialize};

/// The time span for the toplist.
///
/// For example, [`TopRange::OneDay`] will show the top wallpapers in the last 24h.
///
/// This cannot be customized further as only values allowed by wallhaven can be used
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum ToplistRange {
    /// A time span of one day
    #[serde(rename = "1d")]
    OneDay,
    /// A time span of three days
    #[serde(rename = "3d")]
    ThreeDays,
    /// A time span of one week
    #[serde(rename = "1w")]
    OneWeek,
    /// A time span of one month
    #[serde(rename = "1M")]
    OneMonth,
    /// A time span of three months
    #[serde(rename = "3M")]
    ThreeMonths,
    /// A time span of six months
    #[serde(rename = "6M")]
    SixMonths,
    /// A time span of one year
    #[serde(rename = "1y")]
    OneYear,
}
