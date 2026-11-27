# Termux + VNC setup

This is an experimental community route, not a native Android desktop. LDE speaks the X11 protocol. It needs a Linux userspace with Xlib plus a VNC server that exposes an X display. Termux alone is not a replacement for an X server, and LDE does not implement Android's native window system or Wayland.

## Outline

1. Install Termux from a maintained source and install a VNC client on Android.
2. Set up a Linux userspace/proot that supplies a normal Linux distribution (for example Debian). Commands below are run **inside the Linux userspace**, not necessarily in the outer Termux package environment.
3. Install build tools, X11 libraries, VNC, and a couple of applications:

   ```sh
   apt update
   apt install build-essential pkg-config libx11-dev rustc cargo tigervnc-standalone-server xterm pcmanfm rofi
   ```

   Package availability varies by proot image and CPU architecture. If the distro Rust package is too old, install a current Rust toolchain inside that same userspace.
4. Clone/build/install LDE in the proot:

   ```sh
   git clone https://github.com/lemonys-001/lemon-de.git
   cd lemon-de
   cargo build --release --locked
   ```
5. Start a VNC X server. Exact command-line options vary by VNC implementation; one common TigerVNC example is:

   ```sh
   vncserver :1 -geometry 1280x720 -depth 24
   ```

6. Connect the Android VNC viewer to the server. In a shell belonging to the same Linux userspace, select the VNC display and start LDE:

   ```sh
   export DISPLAY=:1
   ./target/release/lemon --no-menu
   ```

   Use the actual VNC display number if it is not `:1`. In a VNC startup script, make the window manager the foreground session process, for example `exec /path/to/lemon --no-menu`; do not background it and let the startup script exit. The 0.3.1+ startup routing also detects `DISPLAY` when Lemon is started from an interactive terminal.

   Native app windows let the window manager assign focus after mapping, avoiding X11 BadMatch errors. LDE also avoids taking exclusive button events away from clients; Alt-drag uses passive grabs instead, preventing BadAccess failures that could terminate the window manager. Desktop apps are launched in separate process groups, reducing the chance that VNC wrappers which clean up a child process group also terminate Lemon. Keep the VNC server and Lemon in the same Linux userspace and do not put a single application as the final xstartup command.

## Notes and limitations

- If the VNC desktop shows no panel, check that `DISPLAY` points to the VNC display and that no other WM is running there.
- Install `xterm` (or set `TERMINAL`) before expecting Super+Enter to work. `rofi` or `dmenu` provides a proper app launcher; the fallback only prints help in a terminal.
- Performance depends on the Android device, proot filesystem, VNC encoding and network. “Lightweight” does not mean zero latency over VNC.
- Android GPU acceleration, audio, battery indicators, touch gestures and Android notifications are not provided by this preview.
- Never expose a VNC server to an untrusted network without authentication and appropriate network protections.

For a desktop Linux install, use [INSTALL.md](INSTALL.md) instead. Please share your device model, Android version, proot distro, architecture and logs when reporting issues.
