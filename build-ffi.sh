#!/usr/bin/env bash
# build-ffi.sh
set -euo pipefail

############################
# 0. Configure constants
############################

echo ">>> Configuring constants"

CRATE_NAME="mutheors"
BUILD_TYPE="release"          # or "debug"
OUT_DIR=".build"
DIST_DIR="dist"
FRAMEWORK_NAME="Mutheors"

# Target triple / artifact suffix / whether to build-std
BUILD_TARGETS=(
    aarch64-apple-ios
    x86_64-apple-ios
    aarch64-apple-ios-sim
    x86_64-apple-darwin
    aarch64-apple-darwin
    x86_64-apple-ios-macabi
    arm64_32-apple-watchos
    aarch64-apple-watchos
    armv7k-apple-watchos
    aarch64-apple-watchos-sim
    x86_64-apple-watchos-sim
)
COPY_SUFFIXES=(
    ios-arm64
    ios-x64
    ios-arm64-sim
    macos-x64
    macos-arm64
    maccatalyst-x64
    watchos-arm64_32
    watchos-arm64
    watchos-armv7k
    watchos-arm64-sim
    watchos-x64-sim
)
USE_ZBUILDSTD=(
    false false false false   # iOS Tier-2
    false false
    true true true true true  # watchOS Tier-3
)


RELFLAG="--release"
FFI_TARGET="$CRATE_NAME"

############################
# 1. Prepare environment
############################

echo ">>> Checking environment"

check_watchos_sdk() {
    if ! xcrun --sdk watchos --show-sdk-path >/dev/null 2>&1; then
        echo "❌ watchOS SDK not found"
        exit 1
    fi
}
check_watchos_sdk

rustup toolchain add nightly
rustup component add rust-src --toolchain nightly

# 👇 Only add USE_ZBUILDSTD=false targets
TIER2_TARGETS=()
for i in "${!BUILD_TARGETS[@]}"; do
    [[ "${USE_ZBUILDSTD[$i]}" == "false" ]] && TIER2_TARGETS+=("${BUILD_TARGETS[$i]}")
done
rustup target add "${TIER2_TARGETS[@]}"

############################
# 2. Prepare build directory
############################

echo ">>> Cleaning old artifacts"

rm -rf "$OUT_DIR" "$DIST_DIR"
mkdir -p "$OUT_DIR" "$OUT_DIR"/swift "$DIST_DIR"

############################
# 3. Build targets & Copy
############################

echo ">>> Building targets"

declare -a FRAMEWORK_ARGS
for i in "${!BUILD_TARGETS[@]}"; do
    TARGET_TRIPLE="${BUILD_TARGETS[$i]}"
    SUFFIX="${COPY_SUFFIXES[$i]}"
    ZBUILDSTD="${USE_ZBUILDSTD[$i]}"

    echo ">>> Building $TARGET_TRIPLE (build-std: $ZBUILDSTD)"
    ZBUILDFLAG=""
    if [ "$ZBUILDSTD" = "true" ]; then
        ZBUILDFLAG="-Zbuild-std=std,panic_abort"
    fi

    # Set SDK / deployment target
    case "$TARGET_TRIPLE" in
        *-apple-ios)           SDK=iphoneos          DEPLOY=10.0 ;;
        *-apple-ios-sim)       SDK=iphonesimulator   DEPLOY=10.0 ;;
        *-apple-darwin)        SDK=macosx            DEPLOY=10.12 ;;
        *-apple-ios-macabi)    SDK=macosx            DEPLOY=10.15 ;;
        *-apple-watchos)       SDK=watchos           DEPLOY=6.0 ;;
        *-apple-watchos-sim)   SDK=watchsimulator    DEPLOY=6.0 ;;
    esac
    export SDKROOT=$(xcrun --sdk "$SDK" --show-sdk-path)
    export "$(tr '[:lower:]' '[:upper:]' <<< "${SDK}")_DEPLOYMENT_TARGET=$DEPLOY"

    # Build
    if [ "$ZBUILDSTD" = "true" ]; then
        cargo +nightly build $RELFLAG $ZBUILDFLAG --target "$TARGET_TRIPLE" --features bindgen
    else
        cargo build $RELFLAG --target "$TARGET_TRIPLE" --features bindgen
    fi

    # Copy & record path
    LIB_PATH="target/$TARGET_TRIPLE/$BUILD_TYPE/lib$FFI_TARGET.a"
    OUT_LIB="$OUT_DIR/lib$FFI_TARGET"_"$SUFFIX".a
    cp "$LIB_PATH" "$OUT_LIB"
    FRAMEWORK_ARGS+=(-library "$OUT_LIB" -headers "$OUT_DIR/swift")
done

############################
# 4. Merge fat libraries
############################
echo "▸ Merging fat libraries"
# macOS universal
lipo -create \
    "$OUT_DIR/lib${FFI_TARGET}_macos-x64.a" \
    "$OUT_DIR/lib${FFI_TARGET}_macos-arm64.a" \
    -output "$OUT_DIR/lib${FFI_TARGET}_macos-universal.a"
# watchOS device universal
lipo -create \
    "$OUT_DIR/lib${FFI_TARGET}_watchos-arm64_32.a" \
    "$OUT_DIR/lib${FFI_TARGET}_watchos-arm64.a" \
    "$OUT_DIR/lib${FFI_TARGET}_watchos-armv7k.a" \
    -output "$OUT_DIR/lib${FFI_TARGET}_watchos-device-universal.a"
# watchOS sim universal
lipo -create \
    "$OUT_DIR/lib${FFI_TARGET}_watchos-arm64-sim.a" \
    "$OUT_DIR/lib${FFI_TARGET}_watchos-x64-sim.a" \
    -output "$OUT_DIR/lib${FFI_TARGET}_watchos-sim-universal.a"
# iOS universal
lipo -create \
    "$OUT_DIR/lib${FFI_TARGET}_ios-arm64-sim.a" \
    "$OUT_DIR/lib${FFI_TARGET}_ios-x64.a" \
    -output "$OUT_DIR/lib${FFI_TARGET}_ios-sim-universal.a"
############################
# 5. Generate Swift bindings
############################
echo "▸ Generating Swift bindings"
cargo build $RELFLAG --features bindgen
cargo run --features bindgen --bin uniffi-bindgen -- \
    generate --library "target/$BUILD_TYPE/lib${FFI_TARGET}.a" \
    --language swift --out-dir "$OUT_DIR/swift"

# rename modulemap
mv "$OUT_DIR/swift/${FFI_TARGET}FFI.modulemap" "$OUT_DIR/swift/module.modulemap"
cp "$OUT_DIR/swift/${FFI_TARGET}.swift" "$DIST_DIR/"

############################
# 6. Create XCFramework
############################
echo "▸ Creating $FRAMEWORK_NAME.xcframework"
xcodebuild -create-xcframework \
    -library "$OUT_DIR/lib${FFI_TARGET}_ios-arm64.a"                -headers "$OUT_DIR/swift" \
    -library "$OUT_DIR/lib${FFI_TARGET}_ios-sim-universal.a"        -headers "$OUT_DIR/swift" \
    -library "$OUT_DIR/lib${FFI_TARGET}_macos-universal.a"          -headers "$OUT_DIR/swift" \
    -library "$OUT_DIR/lib${FFI_TARGET}_maccatalyst-x64.a"          -headers "$OUT_DIR/swift" \
    -library "$OUT_DIR/lib${FFI_TARGET}_watchos-device-universal.a" -headers "$OUT_DIR/swift" \
    -library "$OUT_DIR/lib${FFI_TARGET}_watchos-sim-universal.a"    -headers "$OUT_DIR/swift" \
    -output "$DIST_DIR/$FRAMEWORK_NAME.xcframework"

echo "✅ Done: $DIST_DIR/$FRAMEWORK_NAME.xcframework"
