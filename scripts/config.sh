#!/usr/bin/env bash

set -o allexport; source .env.local; set +o allexport

work_dir="$PWD"
temp_dir="$work_dir/.tmp"
downloads_dir="$temp_dir/downloads"
bin_dir="$temp_dir/bin"

host_arch=$(uname -m)
host_os=$(uname -s)

target_arch="aarch64-unknown-linux-musl"
target_linker="$target_arch-gcc"
export CC_aarch64_unknown_linux_musl="$target_linker"

github_api_repos_base_url="https://api.github.com/repos"

sccache_release_info_url="$github_api_repos_base_url/mozilla/sccache/releases/latest"






