# Lemon components and terminal tools

The `lemon` executable includes an interactive terminal menu and several small Rust applets. Run `lemon` from a terminal without `DISPLAY` to open the menu. If `DISPLAY` is set—including when launched from a Termux/VNC terminal—`lemon` starts the X11 desktop. Use `lemon --menu` to force the menu or `lemon --no-menu` to explicitly start the desktop.

## Built into the Rust binary

- **Lemon Launcher** (`lemon --launcher`): filters executable commands available on `PATH` and starts the selected command. This is a terminal launcher; it is not a graphical `Super+D` menu.
- **Lemon Files** (`lemon --files [PATH]`): browses directories, enters folders, and opens files with the configured file-manager command or `xdg-open` when available. This is a lightweight terminal file browser, not a full graphical file manager.
- **Lemon Settings** (`lemon --settings`, or menu item 3): updates app commands, panel placement and colors, default layout, wallpaper path, and autostart commands in the LDE config. `Super+I` retains the graphical-session editor fallback.
- **Lemon window manager and panel**: the existing X11 shell remains the primary graphical desktop component.

These tools use the Rust standard library; the window manager links against system Xlib. They are deliberately small and do not bundle a browser, terminal emulator, graphical file manager, notification daemon, or other large desktop applications. The interactive installer can install optional system packages for these roles; they remain separate programs and packages.

## Install optional system packages

Choose **Install additional desktop packages** in the menu or run `lemon --install`. The menu detects apt, dnf, pacman, zypper, apk, or Termux `pkg`, shows the package names it will request, and asks for confirmation before using the system package manager. It requests administrator privileges through `sudo` where available on conventional Linux distributions. Termux installs run without `sudo`.

The catalog includes a terminal emulator, file manager, browser, graphical launcher, notification daemon, and wallpaper helper. Package availability and names vary across distributions; review the displayed packages and the package manager's output. On Termux, some X11 packages may require the `x11-repo` repository and an active VNC/X11 session. Installing packages modifies the operating system and is an explicit user action.

After a successful install, Lemon asks whether newly installed terminal, launcher, file manager, or browser commands should become the corresponding LDE defaults. Defaults can also be changed later in Lemon Settings. Commands are saved as user shell command lines in `~/.config/lde/config`; only set commands you trust.

## Configuration

Settings are stored at `$XDG_CONFIG_HOME/lde/config`, or `~/.config/lde/config` when `XDG_CONFIG_HOME` is unset. Run `lemon --config-path` to print the path. See [CONFIGURATION.md](CONFIGURATION.md) for supported keys and desktop shortcuts.
