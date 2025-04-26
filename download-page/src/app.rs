use crate::components::release_links::ReleaseLinks;
use crate::github_repo_url;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let github_repo = github_repo_url();

    html! {
        <div class="download-page">
            <div class="download-page__container">
                <div class="download-page__header">
                    <h1 class="download-page__title">{"Konnektoren Mobile App"}</h1>
                    <img src="/assets/favicon.png" alt="Konnektoren Logo" class="download-page__logo" />
                    <p class="download-page__subtitle">
                        {"Download the latest version of Konnektoren Mobile App for your preferred platform."}
                    </p>
                </div>

                <ReleaseLinks />

                <a
                    href={github_repo}
                    target="_blank"
                    class="download-page__github-link"
                >
                    {"View all releases on GitHub →"}
                </a>

                <footer class="download-page__footer">
                    <p>{"© 2024-2025 Konnektoren Mobile App. All rights reserved."}</p>
                    <p>{"Built with Rust, Tauri, and Yew"}</p>
                </footer>
            </div>
        </div>
    }
}
