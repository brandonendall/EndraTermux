#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
echo "EndraTermux: install Smart Push Run011"
[ -n "${PREFIX:-}" ] || { echo "Run inside Termux" >&2; exit 1; }
[ -f "bin/push" ] || { echo "Run from EndraTermux repository root" >&2; exit 1; }
command -v nlink-v2 >/dev/null || echo "WARNING: nlink-v2 is not installed yet."
install -m 700 bin/push "$PREFIX/bin/push"
echo "Installed: $PREFIX/bin/push"
echo "Usage: push /absolute/path/to/file.tns [/USBTEST]"
echo "Default destination: calculator root (/)"
