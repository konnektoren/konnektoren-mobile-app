# List available commands
default:
    @just --list

# Initialize Android setup
android-init:
    cargo tauri android init

# Initialize iOS setup
ios-init:
    cargo tauri ios init

# Run desktop development server
dev:
    cargo tauri dev

# Run Android development server
android-dev:
    cargo tauri android dev

# Run iOS development server
ios-dev:
    cargo tauri ios dev

# Build desktop application for production
build:
    cargo tauri build

# Build Android application for production
android-build:
    cargo tauri android build

# Build iOS application for production
ios-build:
    cargo tauri ios build

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist
    rm -rf target

# Format code
format:
    cargo fmt

# Run tests
test:
    cargo test
