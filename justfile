# justfile

# Import styles justfile
styles := "styles"

# Android keystore configuration (customize these values)
keystore_path := env_var_or_default("ANDROID_KEYSTORE_PATH", "android_keystore.jks")
keystore_pass := env_var_or_default("ANDROID_KEYSTORE_PASS", "")
key_alias := env_var_or_default("ANDROID_KEY_ALIAS", "konnektoren")
key_pass := env_var_or_default("ANDROID_KEY_PASS", "")

# List available commands
default:
    @just --list

# Setup styles
setup-styles:
    cd {{styles}} && just setup-vendors

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

# Generate a new Android keystore for signing
android-create-keystore:
    #!/usr/bin/env bash
    if [ -f "{{keystore_path}}" ]; then
        echo "Keystore already exists at {{keystore_path}}"
        exit 1
    fi

    if [ -z "{{keystore_pass}}" ] || [ -z "{{key_alias}}" ] || [ -z "{{key_pass}}" ]; then
        echo "Please set the environment variables for keystore config:"
        echo "ANDROID_KEYSTORE_PASS, ANDROID_KEY_ALIAS, ANDROID_KEY_PASS"
        exit 1
    fi

    keytool -genkey -v -keystore {{keystore_path}} \
        -keyalg RSA -keysize 2048 -validity 10000 \
        -alias {{key_alias}} \
        -storepass {{keystore_pass}} -keypass {{key_pass}} \
        -dname "CN=Konnektoren Mobile App, OU=Development, O=Konnektoren, L=Unknown, S=Unknown, C=US"

    echo "Keystore created successfully at {{keystore_path}}"

# Sign an Android APK
android-sign-apk:
    #!/usr/bin/env bash
    if [ ! -f "{{keystore_path}}" ]; then
        echo "Keystore not found at {{keystore_path}}, create one with 'just android-create-keystore'"
        exit 1
    fi

    if [ -z "{{keystore_pass}}" ] || [ -z "{{key_alias}}" ] || [ -z "{{key_pass}}" ]; then
        echo "Please set the environment variables for keystore config:"
        echo "ANDROID_KEYSTORE_PASS, ANDROID_KEY_ALIAS, ANDROID_KEY_PASS"
        exit 1
    fi

    APK_PATH="src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk"
    SIGNED_APK_PATH="src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-signed.apk"

    if [ ! -f "$APK_PATH" ]; then
        echo "Unsigned APK not found at $APK_PATH"
        echo "Build it first with 'just android-build'"
        exit 1
    fi

    # Sign the APK
    jarsigner -sigalg SHA256withRSA -digestalg SHA-256 \
        -keystore {{keystore_path}} -storepass {{keystore_pass}} \
        -keypass {{key_pass}} "$APK_PATH" {{key_alias}}

    # Verify the signature
    jarsigner -verify -verbose -certs "$APK_PATH"

    # Optimize with zipalign (requires Android SDK)
    zipalign -v 4 "$APK_PATH" "$SIGNED_APK_PATH"

    echo "Signed APK available at: $SIGNED_APK_PATH"

# Sign an Android AAB (Android App Bundle)
android-sign-aab:
    #!/usr/bin/env bash
    if [ ! -f "{{keystore_path}}" ]; then
        echo "Keystore not found at {{keystore_path}}, create one with 'just android-create-keystore'"
        exit 1
    fi

    if [ -z "{{keystore_pass}}" ] || [ -z "{{key_alias}}" ] || [ -z "{{key_pass}}" ]; then
        echo "Please set the environment variables for keystore config:"
        echo "ANDROID_KEYSTORE_PASS, ANDROID_KEY_ALIAS, ANDROID_KEY_PASS"
        exit 1
    fi

    AAB_PATH="src-tauri/gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab"
    SIGNED_AAB_PATH="src-tauri/gen/android/app/build/outputs/bundle/universalRelease/app-universal-release-signed.aab"

    if [ ! -f "$AAB_PATH" ]; then
        echo "Unsigned AAB not found at $AAB_PATH"
        echo "Build it first with 'just android-build'"
        exit 1
    fi

    # Sign the AAB
    jarsigner -sigalg SHA256withRSA -digestalg SHA-256 \
        -keystore {{keystore_path}} -storepass {{keystore_pass}} \
        -keypass {{key_pass}} "$AAB_PATH" {{key_alias}}

    # Verify the signature
    jarsigner -verify -verbose -certs "$AAB_PATH"

    # Make a copy of the signed file for clarity
    cp "$AAB_PATH" "$SIGNED_AAB_PATH"

    echo "Signed AAB available at: $SIGNED_AAB_PATH"

# Build and sign Android release (both APK and AAB)
android-release: android-build android-sign-apk android-sign-aab
    @echo "Android release build and signing complete"

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist
    rm -rf target

# Update all dependencies
update: update-rust update-styles

# Update Rust dependencies
update-rust:
    cargo update

# Update style dependencies
update-styles:
    cd {{styles}} && just update-vendors

# Format code
format:
    cargo fmt

# Run tests
test:
    cargo test
