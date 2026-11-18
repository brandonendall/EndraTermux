# EndraTermux — Assistant Project Memory / Handoff

**Read this file when the user says "access your memory" about EndraTermux.** Treat this repository as the source of truth; verify current branches, releases, workflow artifacts and actual files before making claims. This file documents preferences and known workflow, not a guarantee of tool capabilities.

## Project distinction
- **EndraTermux**: Android Termux ARM64 USB command-line tools to communicate with TI-Nspire CX II CAS. Not the separate EndraLink Android APK.
- Working command `nlink` must be preserved; never overwrite it with an experimental diagnostic.
- User uses a Samsung Android phone, Termux and OTG.
- For Termux command blocks start with `clear`.
- User strongly prefers downloadable artifacts **attached directly in ChatGPT** (`sandbox:/mnt/data/...`), rather than GitHub navigation. Only promise this when the file was actually retrieved into the current chat environment. Never invent attachment links or claim a file was downloaded if it was not.
- If direct attachment is unavailable, give a clearly identified verified GitHub raw/artifact link and explain limitations.
- The user-provided main-branch ZIP: `EndraTermux_Run004_Source_And_Binary.zip` at `https://github.com/brandonendall/EndraTermux/blob/main/EndraTermux_Run004_Source_And_Binary.zip`. This is **Run004**, not Build008.
- Prior verified GitHub Actions Build008: run 44206961163, artifact ID 14383952372, `EndraTermux_USB_Recovery_V2_aarch64`, successful Android ARM64 compilation; source branch `feature/endra-usb-recovery-v2`. Artifact URLs may require GitHub sign-in and expire; verify before giving instructions.
- The user has not confirmed downloading/installing/testing Build008. Do not say otherwise.

## Safety and diagnostics
- Goal: read-only USB Recovery V2 diagnostic, no write/reset, bounded timeouts, minimal hardware wear. Diagnose USB filesystem busy and enumeration stalling around the fifteenth directory; consider safe skip-and-continue testing.
- Preserve working binaries and original files. Verify ZIP contents and executable names before installation; never assume an executable is named `endra-usb-recovery-v2`.
- Termux USB command syntax for USB file-descriptor access: `termux-usb -r -E -e 'nlink <subcommand> <args>' /dev/bus/usb/001/002`, but USB bus path can change on reconnect. For a new diagnostic, check its CLI help and actual required invocation first.
- The user's calculator upload shortcut `push` takes **only the local source path** (no destination path). Example: `clear\npush /storage/emulated/0/Download/example.txt.tns`.
- Avoid unnecessary repeated tests, resets, writes or forced unmounts on the calculator.

## How to access memory in future sessions
1. Read this file: `PROJECT_MEMORY.md` on `main` of `brandonendall/EndraTermux`.
2. Inspect the repository README, source ZIP, latest branches and workflow runs for updated truth.
3. Check other project repos separately (EndraLink, Nspire-Development-Hub) rather than confusing them.
4. Tell the user precisely what you verified and provide tested, copy-paste Termux commands.


## Verified Termux checkpoint — 2026-11-18
User downloaded and extracted `EndraTermux_Run004_Source_And_Binary.zip` in `~/EndraTermux`. Archive includes `bin/aarch64/nlink`, Rust `source/nlink-termux/src/main.rs`, Cargo files, rusb-debug and libnspire-sys-debug source. The extracted binary is Android ARM64 ELF, Android API 30, NDK r29, ~2.7 MB.
User ran a read-only binary inspection, then copied the installed working binary from `$PREFIX/bin/nlink` to `~/EndraTermux/backups/nlink_WORKING_BACKUP`. Both SHA256 checks matched exactly:
`5bf3261147f5f264d03ab7d523e29beb40ab6e6a056c4c0af26bbdb0de398c57`.
The extracted Run004 `bin/aarch64/nlink` also has this exact hash. **No replacement or modification of installed nlink occurred.** The backup exists and was verified by matching hashes. User wants the backup preserved and the USB Recovery V2 diagnostic built and run as a *separate* executable, never installed over `nlink`.
Next: obtain/verify actual Build008 USB Recovery V2 artifact and executable name; keep existing nlink intact; inspect before any safe, read-only USB test. No Build008 test has been reported by user.
