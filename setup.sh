#!/usr/bin/env bash
set -e

PREFIX="$HOME/.local/bin"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"

mkdir -p "$PREFIX" "$SYSTEMD_USER_DIR"

# Build the Rust binary in release mode
cargo build --release

# Install the binary
cp target/release/hyprlid "$PREFIX/hyprlid"
chmod +x "$PREFIX/hyprlid"

cp systemd/hyprlid.service "$SYSTEMD_USER_DIR/"

systemctl --user daemon-reload
systemctl --user enable --now hyprlid.service

echo "✅ hyprlid installed and started"
