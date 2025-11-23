use serde::{Deserialize, Serialize};

/// The avatar url of someone who uploaded a wallpaper, in different resolutions.
#[derive(Serialize, Deserialize)]
pub struct Avatar {
    /// 200x200 px
    #[serde(rename = "200px")]
    pub px200: String,
    #[serde(rename = "128px")]
    /// 128x128 px
    pub px128: String,
    /// 32x32 px
    #[serde(rename = "32px")]
    pub px32: String,
    /// 20x20 px
    #[serde(rename = "20px")]
    pub px20: String,
}
