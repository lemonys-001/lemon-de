# Development

The executable is one Rust process that calls a small system Xlib ABI subset directly. Its event loop manages root-window map/configure events, input focus, the panel and keyboard grabs. The shell launches external applications as the current user.

```sh
cargo check --locked
cargo build --release --locked
./target/release/lemon --help
```

A working Rust toolchain, C linker and Xlib are required. Test event handling in a disposable/nested X server; do not start a second WM on an important active session. See [CONTRIBUTING.md](https://github.com/lemonys-001/lemon-de/blob/main/CONTRIBUTING.md) and the repository's [architecture notes](https://github.com/lemonys-001/lemon-de/blob/main/docs/architecture.md).
