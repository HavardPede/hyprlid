#!/usr/bin/env bash
set -euo pipefail

# TODO extract this as a configurable value
LAPTOP="eDP-1"

get_monitors() {
  hyprctl -j monitors | jq -r '.[].name'
}

has_external() {
  get_monitors | grep -qv "$LAPTOP"
}

gdbus monitor --system --dest org.freedesktop.login1 | \
while read -r line; do
  case "$line" in
    *"PropertiesChanged"*LidClosed*true*)
      if has_external; then
        hyprctl dispatch dpms off "$LAPTOP"
      fi
      ;;
    *"PropertiesChanged"*LidClosed*false*)
      hyprctl dispatch dpms on "$LAPTOP"
      ;;
  esac
done
