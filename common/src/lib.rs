use serde::{Deserialize, Serialize};

pub const BASE_URL: &str = "http://localhost:8081";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiResponse<T> {
    pub title: String,
    pub content: T,
}

// About Api
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiAboutRespons {
    pub version: String,
    pub description: String,
    pub contributor: Vec<(String, String)>,
    pub tech: Vec<(String, String, String)>,
    pub contact: Vec<(String, String)>,
}
