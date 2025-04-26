use std::env;

fn main() {
    // Get GitHub repository URL from environment variable or use a default
    let github_repo = env::var("GITHUB_REPO").unwrap_or_else(|_| {
        "https://github.com/konnektoren/konnektoren-mobile-app/releases".to_string()
    });

    // Set the URL as a compile-time environment variable
    // This will be accessible via std::env!("GITHUB_REPO") in the code
    println!("cargo:rustc-env=GITHUB_REPO={}", github_repo);

    // Instruct Cargo to rerun the build script if GITHUB_REPO environment variable changes
    println!("cargo:rerun-if-env-changed=GITHUB_REPO");
}
