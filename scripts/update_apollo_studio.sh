#!/usr/bin/env bash

set -o allexport; source .env.local; set +o allexport

source "$PWD/scripts/config.sh"

which -s rover
if [[ $? != 0 ]]; then
  echo "Installing Apollo Rover"
  cd $downloads_dir || exit
      rm -rf rover
      git clone https://github.com/apollographql/rover
      cd rover || exit
      cargo build --release
      ./target/release/rover install
      cd .. || exit
      rm -rf rover
else
  echo "Rover already installed"
fi

cargo run --bin export_sdl










