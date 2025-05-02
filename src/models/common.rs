use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Thumbs {
    pub large: String,
    pub original: String,
    pub small: String,
}
#[derive(Serialize, Deserialize, Default)]
pub enum Categories {
    #[serde(rename = "100")]
    #[default]
    General,
    #[serde(rename = "101")]
    Anime,
    #[serde(rename = "111")]
    People,
}

#[derive(Serialize, Deserialize, Default)]
pub enum Purity {
    #[serde(rename = "100")]
    #[default]
    Sfw,
    #[serde(rename = "110")]
    Sketchy,
    #[serde(rename = "111")]
    Nsfw,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub enum Resolutions {
    #[serde(rename = "1280x720")]
    R1280x720,
    #[serde(rename = "1280x800")]
    R1280x800,
    #[serde(rename = "1280x960")]
    R1280x960,
    #[serde(rename = "1280x1024")]
    R1280x1024,
    #[serde(rename = "3440x1440")]
    R3440x1440,
    #[serde(rename = "1600x900")]
    R1600x900,
    #[serde(rename = "1600x1000")]
    R1600x1000,
    #[serde(rename = "1600x1200")]
    R1600x1200,
    #[serde(rename = "1600x1280")]
    R1600x1280,
    #[serde(rename = "3840x1600")]
    R3840x1600,
    #[serde(rename = "1920x1080")]
    #[default]
    R1920x1080,
    #[serde(rename = "1920x1200")]
    R1920x1200,
    #[serde(rename = "1920x1440")]
    R1920x1440,
    #[serde(rename = "1920x1536")]
    R1920x1536,
    #[serde(rename = "2560x1080")]
    R2560x1080,
    #[serde(rename = "2560x1440")]
    R2560x1440,
    #[serde(rename = "2560x1600")]
    R2560x1600,
    #[serde(rename = "2560x1920")]
    R2560x1920,
    #[serde(rename = "2560x2048")]
    R2560x2048,
    #[serde(rename = "3840x2160")]
    R3840x2160,
    #[serde(rename = "3840x2400")]
    R3840x2400,
    #[serde(rename = "3840x2880")]
    R3840x2880,
    #[serde(rename = "3840x3072")]
    R3840x3072,
}

#[derive(Serialize, Deserialize, Default)]
pub enum AspectRatio {
    #[serde(rename = "landscape")]
    #[default]
    Landscape,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "16x9")]
    R16x9,
    #[serde(rename = "21x9")]
    R21x9,
    #[serde(rename = "9x16")]
    R9x16,
    #[serde(rename = "1x1")]
    R1x1,
    #[serde(rename = "16x10")]
    R16x10,
    #[serde(rename = "32x9")]
    R32x9,
    #[serde(rename = "10x16")]
    R10x16,
    #[serde(rename = "3x2")]
    R3x2,
    #[serde(rename = "48x9")]
    R48x9,
    #[serde(rename = "9x18")]
    R9x18,
    #[serde(rename = "4x3")]
    R4x3,
    #[serde(rename = "5x4")]
    R5x4,
}

// TODO
#[derive(Serialize, Deserialize, Default)]
pub enum Colors {
    #[default]
    #[serde(rename = "660000")]
    Red,
}
