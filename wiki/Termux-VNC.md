# Termux with a VNC server

LDE is an X11 window manager; it does not provide Android graphics or an X server. The supported experimental Android route is Termux + a Linux userspace/proot + VNC. Results vary by device and distro.

1. Install Termux, a VNC server/client and a Debian/Ubuntu-like Linux userspace.
2. Inside the Linux userspace, install `build-essential libx11-dev rustc cargo tigervnc-standalone-server xterm pcmanfm rofi` (package names and Rust versions may differ).
3. Clone LDE and run `cargo build --release --locked`.
4. Start VNC, for example `vncserver :1 -geometry 1280x720 -depth 24` where supported.
5. Connect a VNC viewer, then in the same userspace run `DISPLAY=:1 ./target/release/lemon`. If using a VNC startup script, make `lemon` the session's window manager.

Install a terminal (`xterm`, or set `TERMINAL`) and optionally `rofi` or `dmenu_run`. LDE currently has no touch gestures, Android notifications, battery/audio controls, compositor or Android integration. Protect VNC with authentication and never expose it to an untrusted network.

See the [full Termux guide](https://github.com/lemonys-001/lemon-de/blob/main/docs/TERMUX.md) for troubleshooting and limitations.
