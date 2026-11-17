#!/data/data/com.termux/files/usr/bin/bash
set -u
echo "EndraTermux diagnostics"
printf 'CPU architecture: '; uname -m
for tool in git python termux-usb nlink; do
  if command -v "$tool" >/dev/null 2>&1; then
    echo "[FOUND] $tool: $(command -v "$tool")"
  else
    echo "[MISSING] $tool"
  fi
done
echo "USB device access still requires Android permission approval."
