#!/bin/sh
set -eu

cd "$MESON_DIST_ROOT"
mkdir -p .cargo
cargo vendor --locked --versioned-dirs vendor > .cargo/config.toml
