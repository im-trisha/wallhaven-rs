use std::fmt::Display;

/// Wallhaven file types
#[derive(Clone, Copy, Debug)]
pub enum FileType {
    /// PNG, transparency aware lossless format
    Png,
    /// JPG/JPEG, lossful compression format
    Jpg,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Png => write!(f, "png"),
            Self::Jpg => write!(f, "jpg"),
        }
    }
}
