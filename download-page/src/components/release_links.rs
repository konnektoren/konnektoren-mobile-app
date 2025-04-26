use yew::prelude::*;

// We'll represent different platforms with this struct
struct PlatformRelease {
    name: &'static str,
    filename: &'static str,
    display_name: &'static str,
    icon: &'static str,
}

#[function_component(ReleaseLinks)]
pub fn release_links() -> Html {
    // Mobile platforms
    let mobile_releases = vec![
        PlatformRelease {
            name: "android",
            filename: "konnektoren-mobile-app.apk",
            display_name: "Android APK",
            icon: "📱",
        },
        // Add iOS when available
        // PlatformRelease {
        //     name: "ios",
        //     filename: "konnektoren-mobile-app.ipa",
        //     display_name: "iOS (TestFlight)",
        //     icon: "🍎",
        // },
    ];

    // Desktop platforms
    let desktop_releases = vec![
        PlatformRelease {
            name: "windows",
            filename: "konnektoren-mobile-app.msi",
            display_name: "Windows Installer",
            icon: "🪟",
        },
        PlatformRelease {
            name: "macos",
            filename: "konnektoren-mobile-app.dmg",
            display_name: "macOS Disk Image",
            icon: "🍎",
        },
        PlatformRelease {
            name: "linux-appimage",
            filename: "konnektoren-mobile-app.AppImage",
            display_name: "Linux AppImage",
            icon: "🐧",
        },
        PlatformRelease {
            name: "linux-deb",
            filename: "konnektoren-mobile-app.deb",
            display_name: "Linux Debian Package",
            icon: "🐧",
        },
    ];

    html! {
        <>
            <div class="download-page__card">
                <div class="download-page__card-body">
                    <h2 class="download-page__card-title">{"Mobile"}</h2>
                    <div class="download-page__links">
                        {mobile_releases.into_iter().map(|release| {
                            html! {
                                <a href={release.filename} class="download-page__link">
                                    <span class="download-page__link-icon">{release.icon}</span>
                                    <span>{release.display_name}</span>
                                </a>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>

            <div class="download-page__card">
                <div class="download-page__card-body">
                    <h2 class="download-page__card-title">{"Desktop"}</h2>
                    <div class="download-page__links">
                        {desktop_releases.into_iter().map(|release| {
                            html! {
                                <a href={release.filename} class="download-page__link">
                                    <span class="download-page__link-icon">{release.icon}</span>
                                    <span>{release.display_name}</span>
                                </a>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </>
    }
}
