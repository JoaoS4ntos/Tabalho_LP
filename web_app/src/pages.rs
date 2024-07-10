use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/register", rank = 1)]
pub async fn register_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/register.html")).await.ok()
}

#[get("/cloud" , rank = 2)]
pub async fn cloud_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/cloud.html")).await.ok()
}

#[get("/", rank = 4)]
pub async fn index_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}