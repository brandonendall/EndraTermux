# EndraTermux — Android ↔ TI-Nspire CX II CAS

## Status (2026-11-22)
Development branch: `feature/run013-quiet-push`. The committed `bin/push` remains Run013. The owner has independently installed and hardware-tested a **local** Run014 `push-run014` progress wrapper and Run015 `npull` recursive import; these exact phone scripts are **not yet in this repository**. Do not install Run013 over a working Run014 without making a backup.

**Field results:** Run014 transferred a 131,072-byte test file to `/upload` with a 100% native byte-progress report and 361.96 KB/s. Run015 `npull -r /EndraHome .` imported six files after two initial connection/listing failures. Nested directory recursion, resume verification, cancellation, timeouts, remote checksums and fresh-phone installation remain unverified.

## Install on a new Android phone

Install Termux and Termux:API Android app, grant USB permission when prompted, and run:

```bash
clear
pkg update -y
pkg install -y git termux-api coreutils python
termux-setup-storage
git clone --branch feature/run013-quiet-push https://github.com/brandonendall/EndraTermux.git ~/EndraTermux-Updates
cd ~/EndraTermux-Updates
bash install.sh
```

Existing clone:

```bash
clear
cd ~/EndraTermux-Updates
git pull --ff-only
bash install.sh
```

**Critical:** This repository does not yet bundle a verified, distributable ARM64 `nlink-v2` executable with all required native dependencies. The installer does not build or install that transport. Do **not** substitute old `nlink`. On a new phone, the wrappers alone cannot perform transfers. Preserve `$PREFIX/bin/nlink` and `$PREFIX/bin/nlink-v2` and their hashes when migrating; verify the source/license and Android architecture before distributing native binaries.

## Upload: `push`

```bash
clear
push /storage/emulated/0/Download/Example.tns /upload
```

```bash
clear
push -d /storage/emulated/0/Download/Example.tns /upload
```

`-d` displays diagnostics. The repository's Run013 wrapper is quiet but **does not** include the locally tested Run014 live progress display. Transfer success does not establish remote checksum equality. Never blindly retry ambiguous writes or reset the calculator.

## Download: `npull` (owner-tested phone script; packaging pending)

```bash
clear
npull /EndraHome/EndraHome_Apps.cfg.tns .
```

```bash
clear
npull -r /EndraHome .
```

```bash
clear
npull -d -r /EndraHome .
```

Normal mode hides native diagnostic spam; `-d` exposes it. The local Run015 importer uses `nlink-v2 ls` structured `[V2 ENUM] stage=item_ok ... name=... type=0 size=...` output to enumerate files, then `nlink-v2 download` per file. The user successfully imported six files from `/EndraHome`. The earlier parser failed on plain filenames; use the corrected local script. **This repo installer does not yet install `npull`** until the exact tested version is recovered and committed. A future Run016 should add live progress, safe reconnect handling, verified skip/resume and nested directory tests.

## GBZ1 ROM packing — Run016 source committed

The exact original owner-supplied `tools/gbz.py` (SHA-256 `19311c5e70fab209482fdc9aeaf8281dbb0dd507e1b2ad8b7157e15c0861dc6d`) and its `bin/gbz` Termux launcher are now committed on this development branch. The packer uses zlib-compressed 32 KiB pages, a GBZ1 header and CRC32 verification. The safe installer installs the tool only if it is missing and **never overwrites an existing GBZ installation**.

```bash
clear
gbz pack /storage/emulated/0/Download/GAME.gba
gbz info /storage/emulated/0/Download/GAME.gbz.tns
gbz unpack /storage/emulated/0/Download/GAME.gbz.tns
```

The original owner-recovered Run014 `push`, Run015 `npull` and Android ARM64 `nlink-v2` are available in the separately delivered Run016 portable recovery ZIP, **not yet committed to this GitHub branch**. This branch remains incomplete for fresh-phone USB transfers. Do not distribute copyrighted ROMs. Hardware verification on a new phone is pending.

## Phone migration checklist

1. Export the **working** `push`, `push-run014`, `npull`, `nlink-v2`, GBZ packer, and required native libraries from the old phone, recording `sha256sum` for each. Do not publish binaries until redistribution rights and compatibility are verified.
2. Install the official Termux and Termux:API apps on the new phone and grant Android storage/USB permissions.
3. Clone this repository, run the installer, and **do not overwrite** newer working wrappers with Run013.
4. Install the verified ARM64 native transport and dependencies separately, compare hashes, and run `nlink-v2 --help` before USB tests.
5. Test with a small disposable file in `/upload`; confirm the calculator sees it. Test `npull` into an empty Android directory, compare local hashes, then try a small folder.
6. Avoid repeated calculator resets, OS updates, unsafe hooks, forced deletions and unbounded retries.

## Development priorities

Run016: capture the owner-tested Run014/Run015 phone scripts, inspect and test them; add live progress to `npull`; make directory parsing robust against diagnostic format changes; bound USB stalls and handle Ctrl+C; verify nested directories and skip/resume; add checksum manifests; recover **real** GBZ packer source and dependencies; publish a verified native ARM64 release with license review; make the installer end-to-end on a fresh phone. Preserve all working backups.

See `PROJECT_MEMORY.md` for test history and limitations.
