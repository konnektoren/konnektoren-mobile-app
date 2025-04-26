use crate::download_url;
use crate::services::github_api::AssetSizeCache;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PlatformCardProps {
    pub title: String,
    pub releases: Vec<ReleaseInfo>,
    pub size_cache: Rc<AssetSizeCache>,
}

#[derive(Clone, PartialEq)]
pub struct ReleaseInfo {
    pub name: String,
    pub filename: String,
    pub display_name: String,
    pub icon: String,
    pub description: String,
    pub file_size: Option<String>, // Now optional, will be populated from API
    pub available: bool,
}

#[function_component(PlatformCard)]
pub fn platform_card(props: &PlatformCardProps) -> Html {
    let selected_release = use_state(|| None::<usize>);

    let available_releases: Vec<_> = props
        .releases
        .iter()
        .enumerate()
        .filter(|(_, release)| release.available)
        .collect();

    // Don't render card if no releases are available
    if available_releases.is_empty() {
        return html! {};
    }

    html! {
        <div class="download-page__card">
            <div class="download-page__card-body">
                <h2 class="download-page__card-title">{&props.title}</h2>
                <div class="download-page__links">
                    {available_releases.iter().map(|(idx, release)| {
                        let idx = *idx;
                        let is_selected = selected_release.as_ref() == Some(&idx);
                        let onclick = {
                            let selected_release = selected_release.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                if is_selected {
                                    selected_release.set(None);
                                } else {
                                    selected_release.set(Some(idx));
                                }
                            })
                        };

                        // Check if we have a file size in cache
                        let file_size = props.size_cache.get(&release.filename)
                            .or_else(|| release.file_size.clone())
                            .unwrap_or_else(|| "Fetching size...".to_string());

                        html! {
                            <div class="download-page__release-container">
                                <a
                                    href={download_url(&release.filename)}
                                    class={classes!(
                                        "download-page__link",
                                        is_selected.then_some("download-page__link--active")
                                    )}
                                    onclick={onclick}
                                >
                                    <span class="download-page__link-icon">{&release.icon}</span>
                                    <span>{&release.display_name}</span>
                                </a>

                                {if is_selected {
                                    html! {
                                        <div class="download-page__release-details">
                                            <p class="download-page__release-description">{&release.description}</p>
                                            <div class="download-page__release-meta">
                                                <span class="download-page__release-size">{"Size: "}{file_size}</span>
                                                <a
                                                    href={download_url(&release.filename)}
                                                    class="download-page__download-button"
                                                >
                                                    {"Download"}{" "}{&release.display_name}
                                                </a>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
    }
}
