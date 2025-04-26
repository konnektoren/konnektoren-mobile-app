use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub size: u64,
    pub browser_download_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<ReleaseAsset>,
}

// Cache for file sizes to avoid multiple API calls
#[derive(PartialEq)]
pub struct AssetSizeCache(Rc<RefCell<HashMap<String, String>>>);

impl AssetSizeCache {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(HashMap::new())))
    }

    pub fn get(&self, filename: &str) -> Option<String> {
        self.0.borrow().get(filename).cloned()
    }

    pub fn set(&self, filename: String, size: String) {
        self.0.borrow_mut().insert(filename, size);
    }
}

pub async fn fetch_latest_release(repo: &str) -> Result<Release, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);

    let response = Request::get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| format!("Network error: {:?}", e))?;

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    response
        .json::<Release>()
        .await
        .map_err(|e| format!("JSON parse error: {:?}", e))
}

// Convert bytes to human-readable size
pub fn format_size(size: u64) -> String {
    if size < 1024 {
        return format!("{} B", size);
    } else if size < 1024 * 1024 {
        return format!("{:.1} KB", size as f64 / 1024.0);
    } else if size < 1024 * 1024 * 1024 {
        return format!("{:.1} MB", size as f64 / (1024.0 * 1024.0));
    } else {
        return format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0));
    }
}
