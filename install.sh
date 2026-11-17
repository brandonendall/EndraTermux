#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
echo "=== EndraTermux Run004 installer ==="
if [ -z "${PREFIX:-}" ] || [ ! -x "${PREFIX}/bin/pkg" ]; then
  echo "Run this inside Termux." >&2; exit 1
fi
pkg update -y
pkg install -y git python termux-api coreutils
mkdir -p "$HOME/nlink-termux"
install -m 700 bin/push "$PREFIX/bin/push"
echo "[OK] Endra Push v2 installed at $PREFIX/bin/push"
if [ "$(uname -m)" != "aarch64" ]; then
  echo "[WARN] Recovered NLink binary is for aarch64 only."
elif [ -f bin/aarch64/nlink ]; then
  echo "Found bundled NLink executable."
  echo "Expected SHA256: 5bf3261147f5f264d03ab7d523e29beb40ab6e6a056c4c0af26bbdb0de398c57"
  echo "5bf3261147f5f264d03ab7d523e29beb40ab6e6a056c4c0af26bbdb0de398c57  bin/aarch64/nlink" | sha256sum -c -
  install -m 755 bin/aarch64/nlink "$PREFIX/bin/nlink"
  echo "[OK] NLink installed"
elif command -v nlink >/dev/null 2>&1; then
  echo "[OK] Existing NLink found: $(command -v nlink)"
else
  echo "[PENDING] NLink binary is not yet in the public repo."
  echo "Transfer verified binary and run: bash scripts/install-nlink.sh /path/to/nlink"
fi
bash scripts/verify-install.sh
echo "=== Setup finished. Connect calculator and use: push /path/to/file.tns ==="
