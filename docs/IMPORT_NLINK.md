# Importing the working NLink binary

Run001 does not contain an NLink executable or a Python transfer implementation.
Obtain the verified working NLink binary from the original Termux device or a trusted build artifact.
Check compatibility with `uname -m` and record `sha256sum nlink` before distribution.

After verification, install locally using:
```sh
install -m 755 nlink "$PREFIX/bin/nlink"
```

Previously confirmed USB command:
```sh
termux-usb -r -E -e 'nlink push LOCAL_FILE REMOTE_DESTINATION' /dev/bus/usb/001/002
```

The USB device path changes when devices reconnect. Do not assume /001/002 always applies.
The Termux:API companion app and Android USB permission approval are required.
