# EndraTermux — TI-Nspire USB transfers from Android

**Run004:** the installer now installs Endra Push v2 automatically and installs NLink when its verified ARM64 executable is present. **The NLink binary and recovered Rust source are not yet published to this public repository**, so a fresh phone still needs the NLink binary imported separately. The setup has not yet been tested on a second phone. Public cloning requires **no GitHub login**.

## First-time Termux setup

Install Termux and its matching Termux:API companion application. Then:

```sh
clear
pkg update -y
pkg install -y git python termux-api
git clone https://github.com/brandonendall/EndraTermux.git
cd EndraTermux
bash install.sh
```

The installer installs the `push` command and creates its staging directory. Until the verified ARM64 NLink binary is committed, import it separately with `bash scripts/install-nlink.sh /path/to/nlink`. Once the binary is published, the same installer will install it automatically after checking its SHA-256 checksum.

## Everyday upload — recommended\n\nEndra Push v2 is a **Bash wrapper**, not an NLink subcommand. It stages the file, searches USB paths, probes each device with `nlink ls /`, and uploads to the calculator root using `nlink upload`.\n\n```sh\nclear\npush /storage/emulated/0/Download/example.tns\n```\n\nOr run `push` alone to be prompted for the full `.tns` path. The script requires Termux:API, an installed NLink executable, and `$HOME/nlink-termux` (created by `install.sh`).\n\n## Identify your calculator's USB port

Connect the TI-Nspire by OTG and run:

```sh
clear
termux-usb -l
```

Example device path: `/dev/bus/usb/001/002`. **The path may change after reconnection.** Select the TI-Nspire entry and approve Android's USB access prompt.

## Transfer commands

The recovered Rust source defines **upload**, **download**, **ls**, **copy**, **move**, **mkdir**, **rmdir**, and **upload-os**. It does **not** define a `push` subcommand. Confirm the actual installed binary with `nlink --help` before transferring. Replace the USB path with the one reported by `termux-usb -l`.

**List calculator documents:**
```sh
termux-usb -r -E -e 'nlink ls /documents' /dev/bus/usb/001/002
```

**Upload a file (Rust source command):**
```sh
termux-usb -r -E -e 'nlink upload example.tns /documents' /dev/bus/usb/001/002
```

**Download a file (Rust source command):**
```sh
mkdir -p "$HOME/nspire-downloads"
termux-usb -r -E -e 'nlink download /documents/example.tns "$HOME/nspire-downloads"' /dev/bus/usb/001/002
```
Note: the inner command's `$HOME` expansion depends on the shell launched by Termux:API. For predictable operation, run the download from a known directory and use a relative destination.

**Create a calculator folder:**
```sh
termux-usb -r -E -e 'nlink mkdir /documents/MyFolder' /dev/bus/usb/001/002
```

**Copy or move on calculator:**
```sh
termux-usb -r -E -e 'nlink copy /documents/a.tns /documents/b.tns' /dev/bus/usb/001/002
termux-usb -r -E -e 'nlink move /documents/b.tns /documents/MyFolder/b.tns' /dev/bus/usb/001/002
```

**Important:** `push` is the custom shell script installed in `$PREFIX/bin/push`. It invokes the binary's `nlink upload` command. Do **not** run `nlink push`.

## Interactive USB helper

```sh
bash scripts/nlink-usb.sh ls /documents
```

This lists USB devices and prompts for the correct path; it does not automatically identify TI hardware.

## Diagnostics

```sh
bash scripts/verify-install.sh
command -v nlink
nlink --help
uname -m
sha256sum "$(command -v nlink)"
```

## Source and licensing

Recovered build: Rust `nlink-termux` 0.1.0, with patched `rusb-debug` (MIT) and `libnspire-sys-debug` (GPL-3.0). Preserve license notices and GPL source obligations when distributing binaries. Do not publish unrelated ROMs, calculator files, logs, credentials, or private configuration.

See `docs/RECOVERY.md` and `docs/IMPORT_NLINK.md`.
