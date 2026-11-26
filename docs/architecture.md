# Architecture and development

## Current implementation

`lemon` is one Rust process. `src/main.rs` declares the small subset of the Xlib C ABI it uses and links to system `libX11`. There are no external Rust crates. The event loop selects root-window substructure events, maps/configures client windows, sets focus and draws a simple panel. Programs such as terminals and launchers are intentionally launched as separate user processes rather than embedded.

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
- Use system Xlib for the X11 protocol path; builds need target-compatible Xlib.
- Do not promise portability solely because Rust compiles: the target must have an X server, Xlib ABI, toolchain and usable input/session setup.
- Features that need long-running services should be optional and documented.
