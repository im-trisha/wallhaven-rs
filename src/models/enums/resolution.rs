use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Resolutions you can filter by.
///
/// You can check out [`Resolution::dimensions`] for numerical values.
///
/// This cannot be customized further as only values allowed by wallhaven can be used.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Resolution {
    /// A resolution of 1280 by 720 pixels
    #[serde(rename = "1280x720")]
    R1280x720,
    /// A resolution of 1280 by 800 pixels
    #[serde(rename = "1280x800")]
    R1280x800,
    /// A resolution of 1280 by 960 pixels
    #[serde(rename = "1280x960")]
    R1280x960,
    /// A resolution of 1280 by 1024 pixels
    #[serde(rename = "1280x1024")]
    R1280x1024,
    /// A resolution of 3440 by 1440 pixels
    #[serde(rename = "3440x1440")]
    R3440x1440,
    /// A resolution of 1600 by 900 pixels
    #[serde(rename = "1600x900")]
    R1600x900,
    /// A resolution of 1600 by 1000 pixels
    #[serde(rename = "1600x1000")]
    R1600x1000,
    /// A resolution of 1600 by 1200 pixels
    #[serde(rename = "1600x1200")]
    R1600x1200,
    /// A resolution of 1600 by 1280 pixels
    #[serde(rename = "1600x1280")]
    R1600x1280,
    /// A resolution of 3840 by 1600 pixels
    #[serde(rename = "3840x1600")]
    R3840x1600,
    /// A resolution of 1920 by 1080 pixels
    #[serde(rename = "1920x1080")]
    R1920x1080,
    /// A resolution of 1920 by 1200 pixels
    #[serde(rename = "1920x1200")]
    R1920x1200,
    /// A resolution of 1920 by 1440 pixels
    #[serde(rename = "1920x1440")]
    R1920x1440,
    /// A resolution of 1920 by 1536 pixels
    #[serde(rename = "1920x1536")]
    R1920x1536,
    /// A resolution of 2560 by 1080 pixels
    #[serde(rename = "2560x1080")]
    R2560x1080,
    /// A resolution of 2560 by 1440 pixels
    #[serde(rename = "2560x1440")]
    R2560x1440,
    /// A resolution of 2560 by 1600 pixels
    #[serde(rename = "2560x1600")]
    R2560x1600,
    /// A resolution of 2560 by 1920 pixels
    #[serde(rename = "2560x1920")]
    R2560x1920,
    /// A resolution of 2560 by 2048 pixels
    #[serde(rename = "2560x2048")]
    R2560x2048,
    /// A resolution of 3840 by 2160 pixels
    #[serde(rename = "3840x2160")]
    R3840x2160,
    /// A resolution of 3840 by 2400 pixels
    #[serde(rename = "3840x2400")]
    R3840x2400,
    /// A resolution of 3840 by 2880 pixels
    #[serde(rename = "3840x2880")]
    R3840x2880,
    /// A resolution of 3840 by 3072 pixels
    #[serde(rename = "3840x3072")]
    R3840x3072,
}

impl Resolution {
    /// A shorthand for a match case returning the actual width and height of the resolution
    ///
    /// Basically, for R(W)x(H) will return (W, H)
    #[must_use]
    pub const fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::R1280x720 => (1280, 720),
            Self::R1280x800 => (1280, 800),
            Self::R1280x960 => (1280, 960),
            Self::R1600x900 => (1600, 900),
            Self::R1280x1024 => (1280, 1024),
            Self::R3440x1440 => (3440, 1440),
            Self::R1600x1000 => (1600, 1000),
            Self::R1600x1200 => (1600, 1200),
            Self::R1600x1280 => (1600, 1280),
            Self::R3840x1600 => (3840, 1600),
            Self::R1920x1080 => (1920, 1080),
            Self::R1920x1200 => (1920, 1200),
            Self::R1920x1440 => (1920, 1440),
            Self::R1920x1536 => (1920, 1536),
            Self::R2560x1080 => (2560, 1080),
            Self::R2560x1440 => (2560, 1440),
            Self::R2560x1600 => (2560, 1600),
            Self::R2560x1920 => (2560, 1920),
            Self::R2560x2048 => (2560, 2048),
            Self::R3840x2160 => (3840, 2160),
            Self::R3840x2400 => (3840, 2400),
            Self::R3840x2880 => (3840, 2880),
            Self::R3840x3072 => (3840, 3072),
        }
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = self.dimensions();
        write!(f, "{w}x{h}")
    }
}
