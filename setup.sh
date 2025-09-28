#!/usr/bin/env bash
set -e

PREFIX="$HOME/.local/bin"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"

mkdir -p "$PREFIX" "$SYSTEMD_USER_DIR"

cp hyprlid.sh "$PREFIX/hyprlid.sh"
chmod +x "$PREFIX/hyprlid.sh"

cp systemd/hyprlid.service "$SYSTEMD_USER_DIR/"

systemctl --user daemon-reload
systemctl --user enable --now hyprlid.service

echo "✅ hyprlid installed and started"
