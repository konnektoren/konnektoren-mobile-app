mod app;
mod components;

pub mod prelude {
    pub use crate::app::App;
}

pub fn github_repo_url() -> String {
    std::option_env!("GITHUB_REPO")
        .unwrap_or("https://github.com/konnektoren/konnektoren-mobile-app/releases")
        .to_string()
}
