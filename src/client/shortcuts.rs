use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{Resolutions, Wallpaper};

use super::{Error, WallhavenClient};

impl Wallpaper {
    pub async fn download(&self, client: &WallhavenClient, outdir: PathBuf) -> Result<(), Error> {
        let path = self.path.clone();
        let extension = Path::new(&path)
            .extension()
            .and_then(OsStr::to_str)
            .ok_or(crate::Error::ErrorParsingPath)?;

        let filepath = outdir.join(format!("wallhaven-{}.{extension}", self.id));

        client.download(path, &filepath).await?;

        Ok(())
    }
}

impl Resolutions {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Resolutions::R1280x720 => (1280, 720),
            Resolutions::R1280x800 => (1280, 800),
            Resolutions::R1280x960 => (1280, 960),
            Resolutions::R1600x900 => (1600, 900),
            Resolutions::R1280x1024 => (1280, 1024),
            Resolutions::R3440x1440 => (3440, 1440),
            Resolutions::R1600x1000 => (1600, 1000),
            Resolutions::R1600x1200 => (1600, 1200),
            Resolutions::R1600x1280 => (1600, 1280),
            Resolutions::R3840x1600 => (3840, 1600),
            Resolutions::R1920x1080 => (1920, 1080),
            Resolutions::R1920x1200 => (1920, 1200),
            Resolutions::R1920x1440 => (1920, 1440),
            Resolutions::R1920x1536 => (1920, 1536),
            Resolutions::R2560x1080 => (2560, 1080),
            Resolutions::R2560x1440 => (2560, 1440),
            Resolutions::R2560x1600 => (2560, 1600),
            Resolutions::R2560x1920 => (2560, 1920),
            Resolutions::R2560x2048 => (2560, 2048),
            Resolutions::R3840x2160 => (3840, 2160),
            Resolutions::R3840x2400 => (3840, 2400),
            Resolutions::R3840x2880 => (3840, 2880),
            Resolutions::R3840x3072 => (3840, 3072),
        }
    }
}
