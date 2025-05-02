use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortingType {
    #[default]
    DateAdded,
    Relevance,
    Random,
    Views,
    Favorites,
    Toplist,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortingOrder {
    #[default]
    Desc,
    Asc,
}

#[derive(Serialize, Deserialize, Default)]
pub enum TopRange {
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeek,
    #[default]
    #[serde(rename = "1M")]
    OneMonth,
    #[serde(rename = "3M")]
    ThreeMonths,
    #[serde(rename = "6M")]
    SixMonths,
    #[serde(rename = "1y")]
    OneYear,
}
