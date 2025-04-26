# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Project Structure

This project consists of two main components:
- `konnektoren-mobile-app-ui`: The Yew frontend written in Rust (WebAssembly)
- `konnektoren-mobile-app`: The Tauri backend for desktop and mobile

## Development Commands

This project uses [just](https://github.com/casey/just) as a command runner. Below are the available commands:

### Mobile Setup

```bash
# Initialize Android setup
just android-init

# Initialize iOS setup
just ios-init
```

### Development

```bash
# Run desktop development server
just dev

# Run Android development server
just android-dev

# Run iOS development server
just ios-dev
```

### Building

```bash
# Build desktop application for production
just build

# Build Android application for production
just android-build

# Build iOS application for production
just ios-build
```

### Utilities

```bash
# Clean build artifacts
just clean

# Format code
just format

# Run tests
just test
```

### Installation

If you don't have `just` installed, you can install it using:

- On macOS with Homebrew: `brew install just`
- On Linux: `cargo install just`
- On Windows with Chocolatey: `choco install just`

Or you can use the raw commands directly with cargo and the Tauri CLI.
