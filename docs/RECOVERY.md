# Recovery notes — Run002
The user's Run002 archive includes an installed aarch64 NLink binary, USB-WORKING build, Rust source (nlink-termux 0.1.0), and local patches rusb-debug and libnspire-sys-debug.

The recovered Rust source uses `upload` and `download` subcommands, not necessarily `push`. Check `nlink --help` on the installed executable before transferring.

Known USB invocation uses `termux-usb -r -E -e 'nlink ...' /dev/bus/usb/001/002`, but the path changes. `scripts/nlink-usb.sh` lets the user select a listed USB device and requests Android authorization.

The recovery archive is not published to this public repository until licensing and private-data review is completed. The Python push script has not been located.
