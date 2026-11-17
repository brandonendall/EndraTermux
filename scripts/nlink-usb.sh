#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
if [ "$#" -lt 1 ]; then
  echo "Usage: bash scripts/nlink-usb.sh <nlink arguments...>" >&2
  echo "Example: bash scripts/nlink-usb.sh ls /documents"
  exit 2
fi
command -v termux-usb >/dev/null || { echo "Install termux-api and Termux:API" >&2; exit 1; }
command -v nlink >/dev/null || { echo "Install verified nlink binary first" >&2; exit 1; }
echo "Available USB paths:"
termux-usb -l
echo "Enter the calculator USB path from the list above (example /dev/bus/usb/001/002):"
read -r device
case "$device" in /dev/bus/usb/*) ;; *) echo "Invalid USB path" >&2; exit 1;; esac
# termux-usb invokes the executable directly; safely quote user arguments for the shell command.
quote() { printf "'%s'" "${1//\'/\'\\\'\'}"; }
cmd="nlink"
for arg in "$@"; do cmd="$cmd $(quote "$arg")"; done
termux-usb -r -E -e "$cmd" "$device"
