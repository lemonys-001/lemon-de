# Security and session safety

LDE is an early window-manager prototype, not a hardened security boundary. It starts application processes as the current user and inherits the current environment. Do not run it as root.

- Test under a disposable/nested X server before using it in a normal session.
- Do not run two window managers against the same X root window.
- Use VNC authentication and network protections; the VNC server, not LDE, is responsible for remote access security.
- Review and trust external programs found on `PATH`; LDE starts them by executable name.
- The close shortcut uses XKillClient on the focused X11 resource and may terminate an app without saving.
- A future stable release should add stronger X11 protocol handling, tests, and security review.

Please report security concerns privately through the repository's GitHub security reporting feature if enabled, otherwise contact the maintainers before public disclosure.
