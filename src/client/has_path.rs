use crate::{WallpaperDetails, WallpaperSummary};

pub trait HasPath: Sync {
    fn path(&self) -> &str;
}

impl HasPath for WallpaperDetails {
    fn path(&self) -> &str {
        self.path.as_str()
    }
}

impl HasPath for WallpaperSummary {
    fn path(&self) -> &str {
        self.path.as_str()
    }
}
