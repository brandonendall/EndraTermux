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


## GitHub Actions artifact download capability — verified 2026-11-18
**Important correction to earlier claims that ChatGPT could not download the build:** The connected GitHub tool `mcp__GitHub__download_workflow_artifact` successfully downloaded GitHub Actions artifact ID `14383952372` from `brandonendall/EndraTermux` into the current ChatGPT container as `/mnt/data/EndraTermux_USB_Recovery_V2_Build008.zip`. ChatGPT provided a direct sandbox attachment link: `[Download Build 008](sandbox:/mnt/data/EndraTermux_USB_Recovery_V2_Build008.zip)`. This file is available **in the chat where downloaded**, not guaranteed to persist across future conversations; repeat the connector download in a new session when necessary. Do not falsely claim downloads are impossible before checking this tool.
- The ZIP was locally inspected with `unzip -l`: it contains exactly one executable `nlink-v2` (2,884,841 bytes).
- Download flow: discover GitHub `download_workflow_artifact`; call with repo, artifact ID, account link ID, optional filename; verify file exists under `/mnt/data` and inspect ZIP; provide a **direct sandbox file link**. For newer builds, first discover the latest successful workflow run and its artifact ID; do not assume Build008 is latest.
- User explicitly requires direct ChatGPT attachments whenever possible, not a GitHub Actions webpage.
- **Do not overwrite `$PREFIX/bin/nlink`**. Install `nlink-v2` separately; confirm CLI usage and USB safety before connecting to the calculator.

## Run011 field checkpoint — 2026-11-19
- Run011 Smart Push is a Bash script `bin/push`, not Python; calls separate compiled `nlink-v2`.
- User installed Run011 directly in Termux and successfully transferred `endrahome_run029_desktop.tns` to calculator directory `/upload`.
- First attempt printed `No such device` for USB `/dev/bus/usb/001/002`; repeating the command succeeded. Cause unproven. Do not assert USB permissions as the root cause.
- Run011 accepts `push SOURCE [CALCULATOR_DIRECTORY]`; default remote directory `/`. Earlier statement in this memory that push takes only one argument describes older versions.
- Success of one transfer does not establish byte-for-byte remote verification or resolve the earlier filesystem-busy/directory enumeration stall.

## Run012 development — 2026-11-19
- Working branch: `feature/run012-usb-reliability`. Preserve Run011 on main until tests pass.
- Run012 adds bounded retry (one retry only for recognized USB open failures), fresh USB device lookup, unique temporary staging, local append-only logs at `~/.local/state/endra/push-run012.log`, and `push --diag`.
- No retry on filesystem-busy, timeouts or ambiguous partial writes. Never reset calculator, terminate calculator processes, or alter `nlink-v2` automatically.
- `push --diag` checks local dependencies and lists USB devices; it does NOT prove calculator file service is healthy.
- Successful `nlink-v2 upload` return code is **transport command success**, NOT remote checksum verification.
- EndraTask companion project runs inside TI-Nspire and may export read-only process/USB/filesystem diagnostics. Remote task kill is a future capability, NOT available via the current `push` file-transfer command.
- Next: GitHub Actions Bash validation, test Run012 on Termux with a small disposable file, compare local logs with EndraTask diagnostic reports, then promote to main only after verified.

## User delivery preference — mandatory (2026-11-19)
- For EVERY future EndraTermux build, retrieve the successful GitHub Actions artifact using the connected GitHub artifact download tool and attach the actual downloadable file in the ChatGPT conversation. Verify local file exists and inspect its contents before linking. Do not merely send a GitHub Actions page or raw GitHub link when a direct attachment is possible.
- Always accompany the direct download with concise, tested-as-far-as-possible Termux copy/paste installation and test commands; start every command block with `clear`.
- Default Android Download location: `/storage/emulated/0/Download/`. The GitHub Actions artifact ZIP may contain a nested `.tar.gz`; inspect before installation. Preserve installed working `push` as a backup, and NEVER replace `nlink` or `nlink-v2` with a script archive.
- Run012 GitHub Actions successful run ID 44605920810, artifact ID 14569652719, artifact name `EndraTermux_Run012_SmartPush`. Downloaded into chat as `/mnt/data/EndraTermux_Run012_Build.zip`; outer ZIP contains `EndraTermux_Run012_SmartPush.tar.gz` and `SHA256SUMS.txt`; tarball contains `EndraTermux_Run012/push` and `PROJECT_MEMORY.md`. This is Bash script packaging, not a newly compiled nlink-v2 binary.
