#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
echo "EndraTermux: install verified NLink binary"
if [ -z "${PREFIX:-}" ]; then echo "Run inside Termux" >&2; exit 1; fi
if [ "$(uname -m)" != "aarch64" ]; then echo "Only the recovered aarch64 build is supported" >&2; exit 1; fi
SOURCE="${1:-}"
if [ -z "$SOURCE" ] || [ ! -f "$SOURCE" ]; then
  echo "Usage: bash scripts/install-nlink.sh /path/to/verified/nlink" >&2
  exit 1
fi
file "$SOURCE" 2>/dev/null || true
sha256sum "$SOURCE"
echo "Install this binary to $PREFIX/bin/nlink? Type YES to confirm:"
read -r answer
[ "$answer" = "YES" ] || exit 1
install -m 755 "$SOURCE" "$PREFIX/bin/nlink"
echo "Installed. Run 'nlink --help' to check commands."
