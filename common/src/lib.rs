use serde::{Deserialize, Serialize};

pub const BASE_URL: &str = "http://127.0.0.1:8081";
pub const API_C: &str = "/capi";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApiResponse<T> {
    pub title: String,
    pub content: T,
}

// About Api
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApiAboutRespons { 
    pub name:String,
    pub version: String,
    pub description: String,
    pub contributor: Vec<(String, String)>,
    pub tech: Vec<(String, String, String)>,
    pub contact: Vec<(String, String)>,
}
