# Snapcraft

Registered on 21. September 2024.

Summary: Konnektoren is a language learning app for German grammar.

Description: Konnektoren is a language learning app for German grammar. It uses gamification to make learning fun and engaging. The app is designed to help users learn German grammar by connecting words and phrases in a fun and interactive way.

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

## Install

```bash
sudo snap install ./konnektoren-app_<version>_<arch>.snap --dangerous
```