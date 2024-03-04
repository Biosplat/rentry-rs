use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub content: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub edit_code: String,
    pub url: String,
    pub content_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub content: String,
    pub url: Option<String>,
    pub edit_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEdit {
    pub url: String,
    pub edit_code: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEditResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostResponse {
    pub url: Option<String>,
    pub edit_code: Option<String>,
    pub success: bool,
    pub message: Option<String>,
}