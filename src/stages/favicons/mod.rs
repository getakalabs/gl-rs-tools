use actix_web::{Result, get};
use actix_files::NamedFile;
use std::borrow::Cow;
use std::path::PathBuf;

// Option struct
struct Options<'a> {
    pub asset_path: Cow<'a, str>,
}

// Default implementation for options
impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            asset_path: Cow::Borrowed("./assets/static/media/favicon.ico")
        }
    }
}

// Create favicon fileserver handler
#[get("/favicon.ico/")]
pub async fn stage() -> Result<NamedFile> {
    let options = Options::default();
    let path: PathBuf = options.asset_path.to_string().parse().unwrap();
    Ok(NamedFile::open(path)?)
}