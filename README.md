# EndraTermux — Android Termux → TI-Nspire CX II CAS

## Current development build: Run013 (quiet push)
Run012 was verified on the user's calculator. Run013 adds quiet-by-default transfers and opt-in `-d` transfer diagnostics. Run013 device testing remains pending.

**Important fresh-phone limitation:** The public repository currently provides the `push` Bash wrapper but **does not yet include a verified distributable Android ARM64 `nlink-v2` executable and its required native dependencies**. On a new phone, installation of the wrapper succeeds, but transfers require separately installing that verified transport. Do not confuse older `nlink` with `nlink-v2`. No GitHub credentials are required to clone this public repository.

## Install on Termux (Android)

Install Termux and the matching Termux:API Android app, then run:

```bash
clear
pkg update -y
pkg install -y git termux-api
git clone --branch feature/run013-quiet-push https://github.com/brandonendall/EndraTermux.git ~/EndraTermux-Updates
cd ~/EndraTermux-Updates
bash install.sh
```

If `~/EndraTermux-Updates` already exists as a clone, update without pushing anything:

```bash
clear
cd ~/EndraTermux-Updates
git pull --ff-only
bash install.sh
```

The installer backs up an existing `$PREFIX/bin/push` to `push.backup` and **never replaces** your existing `nlink-v2` executable.

## Transfer to the calculator

```bash
clear
push /storage/emulated/0/Download/example.tns /upload
```

This normally shows only the sending line, errors if any, and a completion notice. It does **not** yet show real-time byte progress.

To enable full diagnostic output for a transfer:

```bash
clear
push -d /storage/emulated/0/Download/example.tns /upload
```

To inspect USB discovery and installed dependencies without transferring:

```bash
clear
push --diag
```

The calculator destination is optional; default is `/`, but `/upload` is the tested destination. The wrapper adds `.tns` if the source filename lacks it. USB device paths are discovered dynamically, and Android may ask for USB permission.

## Status and development notes

- Run012: verified end-to-end transfer and file readability on TI-Nspire CX II CAS OS 6.4.0.74.
- Run013: quiet-mode script and GitHub Actions package workflow; awaiting phone verification.
- Future: publish a verified native transport with source/license compliance and checksum, so the fresh-phone setup can truly work end to end.
- See `PROJECT_MEMORY.md` for project continuity. No GitHub write credentials are needed for public clone/pull.
