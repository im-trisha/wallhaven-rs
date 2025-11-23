use std::{fmt::Display, num::ParseIntError, str::FromStr};

/// Colors you can filter by.
///
/// Color names found thanks to [color-name.com](https://www.color-name.com/hex/).
///
/// This enum's value is just hex, so you can get the R, G, B values with the following calculations:
/// - `let r = (val & 0xFF0000) >> 16`
/// - `let g = (val & 0x00FF00) >> 8`
/// - `let b = (val & 0x0000FF) >> 0`
///
/// This cannot be customized further as only values allowed by wallhaven can be used.
#[derive(Clone, Copy, Debug)]
pub enum Color {
    /// Color with hex code 0x660000
    BloodRed = 0x66_00_00,
    /// Color with hex code 0x990000
    CrimsonRed = 0x99_00_00,
    /// Color with hex code 0xcc0000
    BostonUniversityRed = 0xcc_00_00,
    /// Color with hex code 0xcc3333
    PersianRed = 0xcc_33_33,
    /// Color with hex code 0xea4c88
    DarkPink = 0xea_4c_88,
    /// Color with hex code 0x993399
    CrayolaViolet = 0x99_33_99,
    /// Color with hex code 0x663399
    RebeccaPurple = 0x66_33_99,
    /// Color with hex code 0x333399
    BluePigment = 0x33_33_99,
    /// Color with hex code 0x0066cc
    TrueBlue = 0x00_66_cc,
    /// Color with hex code 0x0099cc
    RichElectricBlue = 0x00_99_cc,
    /// Color with hex code 0x66cccc
    SeaSerpent = 0x66_cc_cc,
    /// Color with hex code 0x77cc33
    RybGreen = 0x77_cc_33,
    /// Color with hex code 0x669900
    Avocado = 0x66_99_00,
    /// Color with hex code 0x336600
    MetallicGreen = 0x33_66_00,
    /// Color with hex code 0x666600
    BronzeYellow = 0x66_66_00,
    /// Color with hex code 0x999900
    DarkYellow = 0x99_99_00,
    /// Color with hex code 0xcccc33
    Pear = 0xcc_cc_33,
    /// Color with hex code 0xffff00
    Yellow = 0xff_ff_00,
    /// Color with hex code 0xffcc33
    Sunglow = 0xff_cc_33,
    /// Color with hex code 0xff9900
    VividGamboge = 0xff_99_00,
    /// Color with hex code 0xff6600
    Orange = 0xff_66_00,
    /// Color with hex code 0xcc6633
    MediumVermilion = 0xcc_66_33,
    /// Color with hex code 0x996633
    Coconut = 0x99_66_33,
    /// Color with hex code 0x663300
    PhilippineBronze = 0x66_33_00,
    /// Color with hex code 0x000000
    Black = 0x00_00_00,
    /// Color with hex code 0x999999
    SpanishGray = 0x99_99_99,
    /// Color with hex code 0xcccccc
    ChineseSilver = 0xcc_cc_cc,
    /// Color with hex code 0xffffff
    White = 0xff_ff_ff,
    /// Color with hex code 0x424153
    Arsenic = 0x42_41_53,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06X}", *self as u32)
    }
}

impl FromStr for Color {
    type Err = ParseIntError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        // let s = s.strip_prefix("#").unwrap_or(s);
        // let x = u32::from_str_radix(s, 16)?;
        // TODO:
        Ok(Self::Arsenic)
    }
}
