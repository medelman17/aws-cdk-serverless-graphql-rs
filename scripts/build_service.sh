#!/bin/bash

## Setup environment variables

set -o allexport; source .env.local; set +o allexport

WORK_DIR="$PWD"
TEMP_DIR="$WORK_DIR/.tmp"
BIN_DIR="$TEMP_DIR/bin"
HOST_ARCHITECTURE=$(uname -m)
HOST_OS=$(uname -s)
TARGET_ARCHITECTURE="aarch64-unknown-linux-musl"
#TARGET="aarch64-unknown-linux-musl"
#HOST="x86_64-unknown-linux-gnu"
LINKER="$TARGET_ARCHITECTURE-gcc"
GITHUB_API_REPOS_BASE_URL="https://api.github.com/repos"
SCCACHE_RELEASE_INFO_URL="$GITHUB_API_REPOS_BASE_URL/mozilla/sccache/releases/latest"
SCCACHE_DOWNLOAD_DIR="$TEMP_DIR/sccache"
SCCACHE_BINARY=""

echo "$TARGET_ARCHITECTURE"

export OPENSSL_ARCH=linux-aarch64
export CC_aarch64_unknown_linux_musl="$LINKER"
#export CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
#export TARGET_CC=aarch64-unknown-linux-musl-gcc
#export OPENSSL_DIR="/opt/homebrew/etc/openssl@3"
#export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@3/lib/pkgconfig"
#  export LDFLAGS="-L/opt/homebrew/opt/openssl@3/lib"
#  export CPPFLAGS="-I/opt/homebrew/opt/openssl@3/include"

## Clean and ensure necessary directory structure

rm -rf $TEMP_DIR
mkdir -p $TEMP_DIR
mkdir -p $SCCACHE_DOWNLOAD_DIR
mkdir -p $BIN_DIR

## Download build-toolchain dependencies

if [[ "$HOST_OS" = "Darwin" ]]; then
    which -s brew
    if [[ $? != 0 ]]; then
        echo "Installing Homebrew"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    else
        echo "Updating Homebrew"
        brew update --quiet
    fi
    brew tap SergioBenitez/osxct --quiet
    brew install $TARGET_ARCHITECTURE wget jq --quiet

    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER="$LINKER"
#    export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc
#    export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++
#    export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar
#    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc

    if [[ "$HOST_ARCHITECTURE" == "arm64" ]]; then
        SCCACHE_BINARY="sccache-v0.2.15-aarch64-apple-darwin"
    else
        SCCACHE_BINARY="sccache-v0.2.15-x86_64-apple-darwin"
    fi
fi

## Install Rust Using Rustup

which -s rustup
if [[ $? != 0 ]]; then
    echo "Installing Rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s
else
    echo "Updating Rust"
#        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s

fi

## Download Shared Compilation Cache

echo "Installing Shared Compilation Cache"
cd "$SCCACHE_DOWNLOAD_DIR" || exit
curl -s "$SCCACHE_RELEASE_INFO_URL" | jq -r ".assets[] | select(.name | contains(\"$SCCACHE_BINARY\")) | .browser_download_url" | wget -q -i -
tar -xf "$SCCACHE_DOWNLOAD_DIR/$SCCACHE_BINARY.tar.gz"
mv $SCCACHE_BINARY/* "$BIN_DIR" && chmod +x $BIN_DIR/*
export RUSTC_WRAPPER=""
cd "$WORK_DIR" || exit
rm -rf "$SCCACHE_DOWNLOAD_DIR"

rustup target add $TARGET_ARCHITECTURE
cargo install toml-cli

for CONFIG in $(find . -maxdepth 2 -name Cargo.toml); do
    PACKAGE_NAME=$(toml get "$CONFIG" package.name)
    PACKAGE_NAME=$(eval "echo $PACKAGE_NAME")
    TARGET_DIR="$WORK_DIR/target/$PACKAGE_NAME"
    ASSET_DIR="$WORK_DIR/assets/$PACKAGE_NAME"
    echo "Building $PACKAGE_NAME"

#    RUSTFLAGS="-C link_arg=-lgcc" cargo build --release --target aarch64-unknown-linux-musl --no-default-features
    cargo build --release \
        --target $TARGET_ARCHITECTURE \
        --package $PACKAGE_NAME \
        --target-dir $TARGET_DIR

    rm -rf "$ASSET_DIR" && mkdir -p "$ASSET_DIR"
    cd "$TARGET_DIR/$TARGET_ARCHITECTURE/release" || exit
    zip -r -X "./lambda.zip" "./bootstrap"
    cp "lambda.zip" "$ASSET_DIR"
done
