// THIRD PARTY RESPONSE

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Error {
    pub message: String,
    pub code: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub filename: String,
    pub name: String,
    pub mime: String,
    pub extension: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThumbData {
    pub filename: String,
    pub name: String,
    pub mime: String,
    pub extension: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub title: String,
    pub url_viewer: String,
    pub url: String,
    pub display_url: String,
    pub width: u32,
    pub height: u32,
    pub size: u32,
    pub time: u64,
    pub expiration: u32,
    pub image: ImageData,
    pub thumb: ThumbData,
    pub delete_url: String,
}

#[derive(Deserialize, Debug)]
pub struct IBBApiResponse {
    pub error: Option<Error>, // Bisa jadi ada error atau tidak
    pub data: Option<Data>, // Data bisa ada atau tidak
}
