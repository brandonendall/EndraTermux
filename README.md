# EndraTermux

Portable Termux tooling for TI-Nspire NLink USB transfers.

## Run003 status
The Termux dependency bootstrap, binary installer, and interactive USB selector are committed. **The verified NLink executable and full Rust source are not yet published.** The source backup includes local patches `rusb-debug` and `libnspire-sys-debug`. Python transfer script remains unlocated.

## Setup
```sh
pkg update -y
pkg install git -y
git clone https://github.com/brandonendall/EndraTermux.git
cd EndraTermux
bash install.sh
```

## Install your existing NLink binary
On the original phone, the working binary is at `$PREFIX/bin/nlink`. On another aarch64 phone, after transferring and verifying the binary, run:
```sh
bash scripts/install-nlink.sh /path/to/verified/nlink
```

## USB transfer
Connect the calculator and approve Android USB access. The helper lists connected USB devices and asks you to select the calculator path:
```sh
bash scripts/nlink-usb.sh ls /documents
```
For file transfers, check `nlink --help` for the exact supported subcommand. The recovered Rust source uses `upload`, not `push`. The previously confirmed command form was `termux-usb -r -E -e 'nlink push ...' /dev/bus/usb/001/002`; behavior may differ between binaries.

See [recovery notes](docs/RECOVERY.md).
