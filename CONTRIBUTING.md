# Contributing to LDE

Thanks for taking a look. LDE is an early prototype; focused bug fixes and careful docs improvements are especially valuable.

## Before opening an issue

- Include LDE version/commit, distribution, CPU architecture, X server/display manager and how you launched it.
- For bugs, include reproducible steps and relevant stderr/X server logs; redact usernames, paths and other private details.
- Feature requests should explain the user problem and whether they should remain optional to keep the base shell small.

## Pull requests

1. Keep changes scoped and explain behavior changes and platform assumptions.
2. Run `cargo fmt` on Rust changes, then `cargo check --locked` and `cargo build --release --locked` where possible.
3. Test X11 behavior in a nested/disposable server when changing event handling. Do not test with root or endanger a daily session.
4. Update user/developer docs when shortcuts, installation or compatibility change.
5. Do not claim architectures/platforms are supported without evidence. Include test details.

The implementation directly calls Xlib, so ABI correctness and event lifetimes matter. Avoid unsound FFI shortcuts; document any new Xlib types/constants and check their upstream ABI definitions.
