#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
echo "EndraTermux: install Endra Push v2"
[ -n "${PREFIX:-}" ] || { echo "Run inside Termux" >&2; exit 1; }
[ -f "bin/push" ] || { echo "Run from EndraTermux repository root" >&2; exit 1; }
mkdir -p "$HOME/nlink-termux"
install -m 700 bin/push "$PREFIX/bin/push"
echo "Installed: $PREFIX/bin/push"
echo "Usage: push /absolute/path/to/file.tns"
echo "Requires a separately installed, verified nlink binary and Termux:API."
