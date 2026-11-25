---
title: Store Asset Capture
type: guide
status: draft
---

# Store Asset Capture

Original capture: Android 15/API 35 emulator (`Medium_Phone_API_35`), English UI, 1080×1920 at 320 dpi.
See `assets/manifest.json` for the captured build metadata.

Build and install a debug APK, then set the store resolution and a clean status bar:

```sh
cargo tauri android build --debug --apk --target aarch64
adb install -r src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
adb shell wm size 1080x1920
adb shell wm density 320
adb shell settings put global sysui_demo_allowed 1
adb shell am broadcast -a com.android.systemui.demo -e command enter
adb shell am broadcast -a com.android.systemui.demo -e command clock -e hhmm 0900
adb shell am broadcast -a com.android.systemui.demo -e command notifications -e visible false
```

Run from the repository root with the desired screen visible. Save to `/tmp` first to avoid a development-server reload.

```sh
adb -s emulator-5554 exec-out screencap -p > /tmp/mobile-screen.png
magick /tmp/mobile-screen.png PNG24:app-stores/assets/phone-screenshots/03-question.png
```

The app uses hash routes, so screens can be opened directly from Chrome DevTools
(`chrome://inspect`, the debug build's WebView is inspectable): `location.hash = '#/challenge/konnektoren-1'`.

Tablet screenshots: `Pixel_Tablet` AVD in landscape, 2560×1440 (16:9) at its native 320 dpi.
Cold-boot it (`emulator -avd Pixel_Tablet -no-snapshot`): a stale snapshot clock breaks HTTPS, so the
Font Awesome icons from the CDN are missing.

```sh
adb shell settings put system accelerometer_rotation 0
adb shell settings put system user_rotation 0
adb shell wm size 2560x1440
```

Use `$ANDROID_HOME/platform-tools/adb` if needed. Afterwards restore the display:

```sh
adb shell am broadcast -a com.android.systemui.demo -e command exit
adb shell wm size reset
adb shell wm density reset
adb shell settings put system accelerometer_rotation 1
```

Re-export the feature graphic:

```sh
rsvg-convert app-stores/assets/feature-graphic-source.svg -o /tmp/mobile-feature.png
magick /tmp/mobile-feature.png PNG24:app-stores/assets/feature-graphic-1024x500.png
```

Re-export the Snap Store banner (must be 3:1):

```sh
rsvg-convert app-stores/assets/snap-banner-source.svg -o /tmp/snap-banner.png
magick /tmp/snap-banner.png PNG24:app-stores/assets/snap-banner-1920x640.png
```

Refresh dimensions, checksums and capture metadata in `assets/manifest.json` after changes.

Related: [[Store Assets]]
