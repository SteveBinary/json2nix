#!/usr/bin/env bash

cd "$(dirname "$0")"

cargo run -- \
  generate \
  --dark 'Catppuccin Mocha' \
  --light 'Catppuccin Latte' \
  --class-prefix j2n- \
  > ../json2nix-web/assets/public/code-highlighting.css
