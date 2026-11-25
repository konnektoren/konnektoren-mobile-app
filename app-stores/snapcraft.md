# Snapcraft

Registered on 21. September 2024.

Summary: Learn German grammar with Konnektoren.

Description: Konnektoren is a language learning app for German learners. Practise German connectors (Konnektoren), articles and other grammar topics with short, game-like challenges.

Features:

- Multiple-choice grammar challenges with instant feedback and sounds
- Example sentences as help for every question
- Results with score and time taken
- Progress is stored locally on your device

## Build

`snapcraft.yaml` targets core24 for amd64 and arm64. The version is synced by `just release`.

```bash
snapcraft expand-extensions   # quick syntax check, no build
snapcraft pack                # builds for the host architecture (in Multipass on macOS)
```

On Apple Silicon, a local build produces only the arm64 snap. For both architectures use
`snapcraft remote-build`, which builds on Launchpad (the source is uploaded there publicly).

## Publish

```bash
snapcraft login
snapcraft upload konnektoren-app_<version>_amd64.snap --release=edge
snapcraft upload konnektoren-app_<version>_arm64.snap --release=edge
```

## Store listing

Upload in the snapcraft.io dashboard (Listing): the banner `assets/snap-banner-1920x640.png`
(3:1; the Play feature graphic `feature-graphic-1024x500.png` is rejected) and the
tablet screenshots from `assets/tablet-screenshots/`.

## Install

```bash
sudo snap install ./konnektoren-app_<version>_<arch>.snap --dangerous
```