#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
echo "=== EndraTermux safe installer (repo Run013; local Run014/015 not bundled) ==="
[[ -n "${PREFIX:-}" && -x "$PREFIX/bin/pkg" ]] || { echo "Run inside Termux." >&2; exit 1; }
pkg install -y git termux-api coreutils python
[[ -f bin/push ]] || { echo "Run from cloned EndraTermux repository root." >&2; exit 1; }
bash -n bin/push
if [[ -f "$PREFIX/bin/push" ]]; then
  echo "[NOTICE] Existing push found at $PREFIX/bin/push."
  echo "[NOTICE] This repo contains older Run013. Your tested Run014 may be newer."
  if [[ "${ENDRA_INSTALL_RUN013:-0}" != "1" ]]; then
    echo "[SAFE] Existing push preserved. To deliberately install Run013, rerun with ENDRA_INSTALL_RUN013=1."
  else
    backup="$PREFIX/bin/push.backup.$(date +%Y%m%d-%H%M%S)"
    cp -p "$PREFIX/bin/push" "$backup"
    install -m 700 bin/push "$PREFIX/bin/push"
    echo "[OK] Run013 installed; previous push backed up to $backup"
  fi
else
  install -m 700 bin/push "$PREFIX/bin/push"
  echo "[OK] Run013 push installed."
fi
if [[ -x "$PREFIX/bin/npull" ]]; then
  echo "[OK] Existing npull preserved (local Run015; not yet packaged in repo)."
else
  echo "[ACTION REQUIRED] npull is not bundled yet; retrieve verified local Run015 script before migrating."
fi
if [[ -x "$PREFIX/bin/nlink-v2" ]]; then
  echo "[OK] nlink-v2 preserved. SHA-256:"
  sha256sum "$PREFIX/bin/nlink-v2"
else
  echo "[ACTION REQUIRED] Verified Android ARM64 nlink-v2 binary/dependencies not bundled."
fi
if [[ -f tools/gbz.py && -f bin/gbz ]]; then
  python -m py_compile tools/gbz.py
  mkdir -p "$HOME/UltraGBA_Tools"
  if [[ -f "$HOME/UltraGBA_Tools/gbz.py" ]]; then
    if cmp -s tools/gbz.py "$HOME/UltraGBA_Tools/gbz.py"; then
      echo "[OK] Existing GBZ source matches repository; preserved."
    else
      echo "[SAFE] Existing GBZ source differs; preserving it. No automatic overwrite."
    fi
  else
    install -m 600 tools/gbz.py "$HOME/UltraGBA_Tools/gbz.py"
    echo "[OK] GBZ source installed."
  fi
  if [[ -e "$PREFIX/bin/gbz" ]]; then
    echo "[SAFE] Existing gbz launcher preserved."
  else
    install -m 700 bin/gbz "$PREFIX/bin/gbz"
    echo "[OK] GBZ launcher installed."
  fi
else
  echo "[ACTION REQUIRED] GBZ files missing from repository."
fi
echo "Usage: push FILE /upload; npull [-d] [-r] REMOTE_PATH [LOCAL_DIRECTORY] (when installed)."
