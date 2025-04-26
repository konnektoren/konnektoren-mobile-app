mod app;
mod components;
pub mod services;

pub mod prelude {
    pub use crate::app::App;
}

// App version from Cargo.toml
pub fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// GitHub repo URL from environment or default
pub fn github_repo_url() -> String {
    std::option_env!("GITHUB_REPO")
        .unwrap_or("https://github.com/konnektoren/konnektoren-mobile-app/releases")
        .to_string()
}

// Get repository owner/name format for API
pub fn github_repo_api_path() -> String {
    let repo_url = github_repo_url();

    // Extract owner/repo from the GitHub URL
    if let Some(path) = repo_url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = path.trim_end_matches("/releases").split('/').collect();
        if parts.len() >= 2 {
            return format!("{}/{}", parts[0], parts[1]);
        }
    }

    // Default fallback
    "konnektoren/konnektoren-mobile-app".to_string()
}

// GitHub release tag based on version
pub fn github_release_tag() -> String {
    format!("app-v{}", app_version())
}

// Generate download URL for a specific file
pub fn download_url(filename: &str) -> String {
    format!(
        "{}/download/{}/{}",
        github_repo_url().trim_end_matches("/releases"),
        github_release_tag(),
        filename
    )
}
