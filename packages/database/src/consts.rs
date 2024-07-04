use once_cell::sync::{Lazy, OnceCell};
use std::path::{Path, PathBuf};

use sea_orm::DatabaseConnection;

pub static WASM_DIR: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("ROOT_DIR")
        .ok()
        .map(|dir| Path::new(&dir).to_path_buf())
        .unwrap_or(std::env::current_dir().unwrap().join("target/wasm32-html"))
});
pub static WEBSITE_RES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = std::env::var("ROOT_DIR")
        .ok()
        .map(|dir| Path::new(&dir).to_path_buf())
        .unwrap_or(std::env::current_dir().unwrap().join("res"));
    path.push("website");
    path
});
pub static IMAGE_RES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = std::env::var("ROOT_DIR")
        .ok()
        .map(|dir| Path::new(&dir).to_path_buf())
        .unwrap_or(std::env::current_dir().unwrap().join("res"));
    path.push("images");
    path
});
pub static DOCUMENT_RES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = std::env::var("ROOT_DIR")
        .ok()
        .map(|dir| Path::new(&dir).to_path_buf())
        .unwrap_or(std::env::current_dir().unwrap().join("res"));
    path.push("documents");
    path
});
pub static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = std::env::var("ROOT_DIR")
        .ok()
        .map(|dir| Path::new(&dir).to_path_buf())
        .unwrap_or(std::env::current_dir().unwrap().join("res"));
    path.push("log");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
});

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("DATABASE_URL")
        .ok()
        .unwrap_or("mysql://kongying:HoM0iI45l4L9198iO@127.0.0.1:3306/kongying".to_string())
});
pub static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::new();
