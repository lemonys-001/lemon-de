# Lemon components and built-in apps

`lemon` includes a terminal menu and small Rust applets. Run `lemon` from a terminal without `DISPLAY` to open the menu. If `DISPLAY` is set—including in Termux/VNC—`lemon` starts the X11 desktop. Use `lemon --menu` to force the menu or `lemon --no-menu` to explicitly start the desktop.

## Native X11 and terminal applets

When a valid `DISPLAY` is available, these commands open lightweight Xlib windows; without it, they use their terminal interfaces:

- **Lemon Launcher** — `lemon --launcher` or **Super+D**. Filters commands on `PATH`, supports keyboard search, and launches the selected command.
- **Lemon Files** — `lemon --files [PATH]` or the panel's **Files** action. Browse directories, enter folders, and open files using the configured file-manager command or `xdg-open`.
- **Lemon Settings** — `lemon --settings` or **Super+I**. Edit app defaults, panel placement/colors, layout, wallpaper, and autostart commands; press Ctrl+S to save in the X11 window.
- **Lemon Calculator** — `lemon --calculator`, **Super+C**, or the panel's **Calc** action. Basic arithmetic, parentheses, keyboard and mouse input.

The applets use system Xlib and the Rust standard library only. They have no toolkit, bundled web engine, animation loop, or persistent background service. A normal LDE session continues to own window management; applets run as ordinary X11 clients so they can be moved/resized with Alt+left/right-drag. Super+M minimizes a client, Super+Shift+M restores the last minimized one, and Alt+Tab cycles visible clients. There are no titlebar buttons yet.

LDE still does not bundle a web browser, graphical terminal emulator, notification daemon, power manager or other large desktop services. Those remain optional separate programs and can be installed from the terminal package menu.

## Install optional system packages

Choose **Install additional desktop packages** in the terminal menu or run `lemon --install`. The menu detects apt, dnf, pacman, zypper, apk, or Termux `pkg`, shows the package names it will request, and asks for confirmation before using the system package manager. It requests administrator privileges through `sudo` where available on conventional Linux distributions. Termux installs run without `sudo`.

The catalog includes a terminal emulator, file manager, browser, graphical launcher, notification daemon, and wallpaper helper. Package availability and names vary across distributions; review the displayed packages and the package manager's output. On Termux, some X11 packages may require the `x11-repo` repository and an active VNC/X11 session. Installing packages modifies the operating system and is an explicit user action.

After a successful install, Lemon asks whether newly installed terminal, launcher, file manager, or browser commands should become the corresponding LDE defaults. Defaults can also be changed later in Lemon Settings. Commands are saved as user shell command lines in `~/.config/lde/config`; only set commands you trust.

## Configuration

Settings are stored at `$XDG_CONFIG_HOME/lde/config`, or `~/.config/lde/config` when `XDG_CONFIG_HOME` is unset. Run `lemon --config-path` to print the path. See [CONFIGURATION.md](CONFIGURATION.md) for supported keys and shortcuts.
