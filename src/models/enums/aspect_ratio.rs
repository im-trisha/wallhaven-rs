use serde::{Deserialize, Serialize};

/// An aspect ratio to filter by.
///
/// You may want to check the wikipedia page for
/// [page orientation](https://en.wikipedia.org/wiki/Page_orientation) and
/// [aspect ratio](https://en.wikipedia.org/wiki/Pixel_aspect_ratio)
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum AspectRatio {
    /// Landscape mode, basically when W > H
    #[serde(rename = "landscape")]
    Landscape,
    /// Portrait mode, basically when H > W
    #[serde(rename = "portrait")]
    Portrait,
    /// 16:9 PAR
    #[serde(rename = "16x9")]
    R16x9,
    /// 21:9 PAR
    #[serde(rename = "21x9")]
    R21x9,
    /// 9:16 PAR
    #[serde(rename = "9x16")]
    R9x16,
    /// 1:1 PAR
    #[serde(rename = "1x1")]
    R1x1,
    /// 16:10 PAR
    #[serde(rename = "16x10")]
    R16x10,
    /// 32:9 PAR
    #[serde(rename = "32x9")]
    R32x9,
    /// 10:16 PAR
    #[serde(rename = "10x16")]
    R10x16,
    /// 3:2 PAR
    #[serde(rename = "3x2")]
    R3x2,
    /// 48:9 PAR
    #[serde(rename = "48x9")]
    R48x9,
    /// 9:18 PAR
    #[serde(rename = "9x18")]
    R9x18,
    /// 4:3 PAR
    #[serde(rename = "4x3")]
    R4x3,
    /// 5:4 PAR
    #[serde(rename = "5x4")]
    R5x4,
}
