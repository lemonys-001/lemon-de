# Architecture and development

## Current implementation

`lemon` is one Rust binary with two independent X11 roles. `src/main.rs` declares the Xlib subset for the window manager and panel; `src/compositor.rs` is an opt-in event-driven X Composite/X Damage/X Fixes/X Render compositor started with `lemon --compositor`. It redirects root children, draws named window pixmaps to the compositor overlay, and relies on Damage plus structure notifications to repaint. The compositor handles alpha-capable visuals only; there are no blur, shadow, animation, or scheduling pipelines. `src/update_checker.rs` caches a daily GitHub release metadata check and launches a bounded curl worker only when due; no resident polling process or update installer is added. There are no external Rust crates. Programs such as terminals, launchers, wallpaper setters, desktop-icon providers and optional notification/power helpers are intentionally launched as separate user processes rather than embedded. The user's simple `~/.config/lde/config` settings are parsed with the standard library; no GUI toolkit or new Rust crates are added.

This is a compact prototype rather than a mature window manager: X11 error handling is intentionally minimal, client protocol coverage is incomplete, placement is simple, and some session behavior depends on the display manager/X server. Do not treat it as a security boundary.

## Build / checks

```sh
cargo fmt
cargo check --locked
cargo build --release --locked
./target/release/lemon --help
```

A host with Rust and the X11 linker library is required. The help/version paths do not open an X server. For end-to-end testing, use a disposable X server or nested Xephyr session; do not replace the active window manager in an important session.

## Design constraints

- Keep the core runtime small and avoid mandatory heavyweight services.
- Use system Xlib and X11 extension libraries for the X11 path; source builds need target-compatible Xlib, XComposite, XDamage, XFixes, and XRender development files.
- Do not promise portability solely because Rust compiles: the target must have an X server, Xlib ABI, toolchain and usable input/session setup.
- Features that need long-running services should be optional and documented.
