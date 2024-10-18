# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Install Tauri

```bash
cargo install create-tauri-app
cargo install tauri-cli
```

Install the pre-requisites for your platform from [here](https://v2.tauri.app/start/prerequisites/).

## Building

Check

``` bash
pwd
```

is the root of the project `konnektoren-mobile-app`.

Build for desktop with

```bash
cargo tauri dev
```

Build for Android with

```bash
cargo tauri android build
```

## Running

```bash
cargo tauri android dev
```
