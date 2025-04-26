use super::platform_card::{PlatformCard, ReleaseInfo};
use crate::app_version;
use crate::github_repo_api_path;
use crate::services::github_api::{AssetSizeCache, fetch_latest_release, format_size};
use gloo::console;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(ReleaseLinks)]
pub fn release_links() -> Html {
    let version = app_version();

    // Create a state for our size cache
    let size_cache = use_state(|| Rc::new(AssetSizeCache::new()));

    // Effect to fetch file sizes
    {
        let size_cache = size_cache.clone();
        use_effect_with((), move |_| {
            let size_cache = Rc::clone(&size_cache);
            spawn_local(async move {
                let repo_path = github_repo_api_path();
                match fetch_latest_release(&repo_path).await {
                    Ok(release) => {
                        console::log!("Fetched release info:", &release.tag_name);

                        // Update our cache with file sizes
                        for asset in release.assets {
                            let size_str = format_size(asset.size);
                            console::log!("Asset:", &asset.name, "Size:", &size_str);
                            size_cache.set(asset.name, size_str);
                        }
                    }
                    Err(e) => {
                        console::error!("Failed to fetch release info:", e);
                    }
                }
            });
            || ()
        });
    }

    // Mobile platforms
    let mobile_releases = vec![
        ReleaseInfo {
            name: "android".to_string(),
            filename: format!("konnektoren-mobile-app-universal-release_{}.apk", version),
            display_name: "Android APK".to_string(),
            icon: "📱".to_string(),
            description: "Android APK file that can be installed directly on your Android device. Requires Android 6.0 or later.".to_string(),
            file_size: None, // No hardcoded size, will be fetched
            available: false, // Set to true when available
        },
        ReleaseInfo {
            name: "ios".to_string(),
            filename: format!("konnektoren-mobile-app_{}_ios.ipa", version),
            display_name: "iOS (TestFlight)".to_string(),
            icon: "🍎".to_string(),
            description: "iOS app available through TestFlight for beta testing. Requires iOS 13.0 or later.".to_string(),
            file_size: None,
            available: false, // Set to true when available
        },
    ];

    // Windows platforms
    let windows_releases = vec![
        ReleaseInfo {
            name: "windows-exe".to_string(),
            filename: format!("konnektoren-mobile-app_{}_x64-setup.exe", version),
            display_name: "Windows Setup (EXE)".to_string(),
            icon: "🪟".to_string(),
            description: "Windows installer executable. Just download and run to install. Supports Windows 10/11.".to_string(),
            file_size: None,
            available: true,
        },
        ReleaseInfo {
            name: "windows-msi".to_string(),
            filename: format!("konnektoren-mobile-app_{}_x64_en-US.msi", version),
            display_name: "Windows MSI Package".to_string(),
            icon: "🪟".to_string(),
            description: "Windows MSI installer package for system administrators. Supports Windows 10/11.".to_string(),
            file_size: None,
            available: true,
        },
    ];

    // macOS platforms
    let macos_releases = vec![
        ReleaseInfo {
            name: "macos-intel".to_string(),
            filename: format!("konnektoren-mobile-app_{}_x64.dmg", version),
            display_name: "macOS Intel (DMG)".to_string(),
            icon: "🍎".to_string(),
            description: "macOS disk image for Intel-based Macs. Supports macOS 10.15 (Catalina) or later.".to_string(),
            file_size: None,
            available: true,
        },
        ReleaseInfo {
            name: "macos-arm".to_string(),
            filename: format!("konnektoren-mobile-app_{}_aarch64.dmg", version),
            display_name: "macOS Apple Silicon (DMG)".to_string(),
            icon: "🍎".to_string(),
            description: "macOS disk image optimized for Apple Silicon (M1/M2/M3). Supports macOS 11 (Big Sur) or later.".to_string(),
            file_size: None,
            available: true,
        },
        ReleaseInfo {
            name: "macos-intel-tar".to_string(),
            filename: "konnektoren-mobile-app_x64.app.tar.gz".to_string(),
            display_name: "macOS Intel (tar.gz)".to_string(),
            icon: "🍎".to_string(),
            description: "Compressed app bundle for Intel Macs. Extract and move to Applications folder.".to_string(),
            file_size: None,
            available: true,
        },
        ReleaseInfo {
            name: "macos-arm-tar".to_string(),
            filename: "konnektoren-mobile-app_aarch64.app.tar.gz".to_string(),
            display_name: "macOS Apple Silicon (tar.gz)".to_string(),
            icon: "🍎".to_string(),
            description: "Compressed app bundle for Apple Silicon Macs. Extract and move to Applications folder.".to_string(),
            file_size: None,
            available: true,
        },
    ];

    // Linux platforms
    let linux_releases = vec![
        ReleaseInfo {
            name: "linux-appimage".to_string(),
            filename: format!("konnektoren-mobile-app_{}_amd64.AppImage", version),
            display_name: "Linux AppImage".to_string(),
            icon: "🐧".to_string(),
            description: "Portable Linux application that runs on most distributions without installation. Just make executable and run.".to_string(),
            file_size: None,
            available: true,
        },
        ReleaseInfo {
            name: "linux-deb".to_string(),
            filename: format!("konnektoren-mobile-app_{}_amd64.deb", version),
            display_name: "Debian/Ubuntu Package".to_string(),
            icon: "🐧".to_string(),
            description: "Package for Debian-based distributions like Ubuntu, Linux Mint, or Pop!_OS. Install with dpkg or apt.".to_string(),
            file_size: None,
            available: true,
        },
        ReleaseInfo {
            name: "linux-rpm".to_string(),
            filename: format!("konnektoren-mobile-app-{}-1.x86_64.rpm", version),
            display_name: "Fedora/RHEL Package".to_string(),
            icon: "🐧".to_string(),
            description: "Package for RPM-based distributions like Fedora, Red Hat, or openSUSE. Install with dnf or rpm.".to_string(),
            file_size: None,
            available: true,
        },
    ];

    html! {
        <>
            <PlatformCard title="Mobile Apps" releases={mobile_releases} size_cache={(*size_cache).clone()} />
            <PlatformCard title="Windows" releases={windows_releases} size_cache={(*size_cache).clone()} />
            <PlatformCard title="macOS" releases={macos_releases} size_cache={(*size_cache).clone()} />
            <PlatformCard title="Linux" releases={linux_releases} size_cache={(*size_cache).clone()} />
        </>
    }
}
