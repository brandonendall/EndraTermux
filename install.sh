#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
echo "EndraTermux Run001: environment bootstrap"
if [ -z "${PREFIX:-}" ] || [ ! -x "${PREFIX}/bin/pkg" ]; then
  echo "Run this inside Termux." >&2; exit 1
fi
pkg update -y
pkg install -y git python termux-api
bash scripts/verify-install.sh
echo "Bootstrap complete. NLink binary must still be imported and verified."
