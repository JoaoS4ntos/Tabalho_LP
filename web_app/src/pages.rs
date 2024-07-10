use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/register")]
pub async fn register_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/register.html")).await.ok()
}

#[get("/cloud")]
pub async fn cloud_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/cloud.html")).await.ok()
}
