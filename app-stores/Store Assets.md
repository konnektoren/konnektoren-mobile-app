---
title: Store Assets
type: guide
status: draft
---

# Store Assets

Assets are under `app-stores/assets/`. Screenshots capture a development build, not release validation.

| File | Content |
|---|---|
| `app-icon.png` | 512×512 icon |
| `feature-graphic-1024x500.png` | 1024×500 feature graphic (Google Play) |
| `feature-graphic-source.svg` | Editable graphic source (mascot embedded) |
| `snap-banner-1920x640.png` | 1920×640 banner for the Snap Store (3:1, upload in the snapcraft.io dashboard) |
| `snap-banner-source.svg` | Editable banner source |
| `phone-screenshots/01-home.png` | Home: mascot, Start Learning, social links |
| `phone-screenshots/02-challenges.png` | Challenge catalogue |
| `phone-screenshots/03-question.png` | Konnektoren question with answer options |
| `phone-screenshots/04-result.png` | Result summary (90 %, time taken) |
| `tablet-screenshots/01-home.png` | Home, landscape tablet |
| `tablet-screenshots/02-challenges.png` | Challenge catalogue in three columns |
| `tablet-screenshots/03-question.png` | Question with answer options |
| `tablet-screenshots/04-result.png` | Result summary (90 %, time taken) |
| `legacy/` | Earlier screenshots, feature and header images from the old `store/` folder |
| `manifest.json` | Capture environment, dimensions and checksums |

Phone screenshots are 1080×1920 and tablet screenshots 2560×1440 (16:9 landscape) opaque PNGs captured from the Android emulator.

- [ ] Review artwork rights and listing copy.
- [ ] Refresh screenshots from a release build once the konnektoren-yew fixes are published.
- [ ] Check current [Play asset requirements](https://support.google.com/googleplay/android-developer/answer/9866151?hl=en).

Related: [[Store Asset Capture]]
