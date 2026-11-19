#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
echo "=== EndraTermux Run013 installer ==="
[[ -n "${PREFIX:-}" && -x "$PREFIX/bin/pkg" ]] || { echo "Run inside Termux." >&2; exit 1; }
pkg install -y git termux-api coreutils
[[ -f bin/push ]] || { echo "Run from cloned EndraTermux repository root." >&2; exit 1; }
bash -n bin/push
if [[ -f "$PREFIX/bin/push" ]]; then
  cp -p "$PREFIX/bin/push" "$PREFIX/bin/push.backup"
  echo "[OK] Previous push backed up to $PREFIX/bin/push.backup"
fi
install -m 700 bin/push "$PREFIX/bin/push"
echo "[OK] Run013 quiet push installed."
if ! command -v nlink-v2 >/dev/null 2>&1; then
  echo "[ACTION REQUIRED] nlink-v2 is missing. A fresh phone cannot transfer until the verified Android ARM64 nlink-v2 binary and its dependencies are installed."
  echo "The repository does not currently bundle a verified nlink-v2 binary; do not substitute the older nlink binary."
else
  echo "[OK] Existing nlink-v2 preserved: $(command -v nlink-v2)"
fi
echo "Normal: push /storage/emulated/0/Download/example.tns /upload"
echo "Debug:  push -d /storage/emulated/0/Download/example.tns /upload"
echo "USB:    push --diag"
