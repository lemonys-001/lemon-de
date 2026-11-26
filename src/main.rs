//! LDE's deliberately small X11 shell. This file uses the Xlib ABI directly so
//! the runtime has no Rust crate dependencies. See docs/architecture.md.
use std::collections::HashSet;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
use std::process::Command;
use std::ptr;

#[repr(C)] struct Display { _private: [u8; 0] }
type Window = c_ulong;
type GC = *mut c_void;

#[repr(C)] #[derive(Clone, Copy)]
struct XKeyEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    window: Window, root: Window, subwindow: Window, time: c_ulong,
    x: c_int, y: c_int, x_root: c_int, y_root: c_int, state: c_uint,
    keycode: c_uint, same_screen: c_int,
}
#[repr(C)] #[derive(Clone, Copy)]
struct XButtonEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    window: Window, root: Window, subwindow: Window, time: c_ulong,
    x: c_int, y: c_int, x_root: c_int, y_root: c_int, state: c_uint,
    button: c_uint, same_screen: c_int,
}
#[repr(C)] #[derive(Clone, Copy)]
struct XMapRequestEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    parent: Window, window: Window,
}
#[repr(C)] #[derive(Clone, Copy)]
struct XConfigureRequestEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    parent: Window, window: Window, x: c_int, y: c_int, width: c_int,
    height: c_int, border_width: c_int, above: Window, detail: c_int,
    value_mask: c_ulong,
}
#[repr(C)] #[derive(Clone, Copy)]
struct XExposeEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    window: Window, x: c_int, y: c_int, width: c_int, height: c_int, count: c_int,
}
#[repr(C)] #[derive(Clone, Copy)]
struct XDestroyWindowEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    event: Window, window: Window,
}
#[repr(C)] union XEvent {
    type_: c_int,
    key: XKeyEvent,
    button: XButtonEvent,
    map_request: XMapRequestEvent,
    configure_request: XConfigureRequestEvent,
    expose: XExposeEvent,
    destroy: XDestroyWindowEvent,
    pad: [c_long; 24],
}
#[repr(C)] struct PollFd { fd: c_int, events: c_short, revents: c_short }

#[repr(C)] struct XSetWindowAttributes {
    background_pixmap: c_ulong, background_pixel: c_ulong, border_pixmap: c_ulong,
    border_pixel: c_ulong, bit_gravity: c_int, win_gravity: c_int,
    backing_store: c_int, backing_planes: c_ulong, backing_pixel: c_ulong,
    save_under: c_int, event_mask: c_long, do_not_propagate_mask: c_long,
    override_redirect: c_int, colormap: c_ulong, cursor: c_ulong,
}

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(name: *const c_char) -> *mut Display;
    fn XCloseDisplay(display: *mut Display) -> c_int;
    fn XDefaultRootWindow(display: *mut Display) -> Window;
    fn XDefaultScreen(display: *mut Display) -> c_int;
    fn XDisplayWidth(display: *mut Display, screen: c_int) -> c_int;
    fn XDisplayHeight(display: *mut Display, screen: c_int) -> c_int;
    fn XCreateSimpleWindow(display: *mut Display, parent: Window, x: c_int, y: c_int,
        width: c_uint, height: c_uint, border_width: c_uint, border: c_ulong,
        background: c_ulong) -> Window;
    fn XChangeWindowAttributes(display: *mut Display, window: Window, mask: c_ulong,
        attributes: *mut XSetWindowAttributes) -> c_int;
    fn XSelectInput(display: *mut Display, window: Window, mask: c_long) -> c_int;
    fn XSetWindowBackground(display: *mut Display, window: Window, pixel: c_ulong) -> c_int;
    fn XClearWindow(display: *mut Display, window: Window) -> c_int;
    fn XMapWindow(display: *mut Display, window: Window) -> c_int;
    fn XMoveResizeWindow(display: *mut Display, window: Window, x: c_int, y: c_int,
        width: c_uint, height: c_uint) -> c_int;
    fn XRaiseWindow(display: *mut Display, window: Window) -> c_int;
    fn XNextEvent(display: *mut Display, event: *mut XEvent) -> c_int;
    fn XPending(display: *mut Display) -> c_int;
    fn XConnectionNumber(display: *mut Display) -> c_int;
    fn poll(fds: *mut PollFd, count: c_ulong, timeout_ms: c_int) -> c_int;
    fn XCreateGC(display: *mut Display, drawable: Window, mask: c_ulong,
        values: *mut c_void) -> GC;
    fn XSetForeground(display: *mut Display, gc: GC, foreground: c_ulong) -> c_int;
    fn XFillRectangle(display: *mut Display, drawable: Window, gc: GC, x: c_int,
        y: c_int, width: c_uint, height: c_uint) -> c_int;
    fn XDrawString(display: *mut Display, drawable: Window, gc: GC, x: c_int,
        y: c_int, string: *const c_char, length: c_int) -> c_int;
    fn XFlush(display: *mut Display) -> c_int;
    fn XSetInputFocus(display: *mut Display, window: Window, revert_to: c_int,
        time: c_ulong) -> c_int;
    fn XSetWindowBorderWidth(display: *mut Display, window: Window, width: c_uint) -> c_int;
    fn XSetWindowBorder(display: *mut Display, window: Window, pixel: c_ulong) -> c_int;
    fn XKillClient(display: *mut Display, resource: c_ulong) -> c_int;
    fn XGrabKey(display: *mut Display, keycode: c_int, modifiers: c_uint,
        grab_window: Window, owner_events: c_int, pointer_mode: c_int,
        keyboard_mode: c_int) -> c_int;
    fn XKeysymToKeycode(display: *mut Display, keysym: c_ulong) -> c_int;
}

const KEY_PRESS: c_int = 2;
const BUTTON_PRESS: c_int = 4;
const EXPOSE: c_int = 12;
const DESTROY_NOTIFY: c_int = 17;
const MAP_REQUEST: c_int = 20;
const CONFIGURE_REQUEST: c_int = 23;
const KEY_PRESS_MASK: c_long = 1;
const BUTTON_PRESS_MASK: c_long = 1 << 2;
const EXPOSURE_MASK: c_long = 1 << 15;
const SUBSTRUCTURE_NOTIFY_MASK: c_long = 1 << 19;
const SUBSTRUCTURE_REDIRECT_MASK: c_long = 1 << 20;
const CW_OVERRIDE_REDIRECT: c_ulong = 1 << 9;
const MOD4_MASK: c_uint = 1 << 6;
const SHIFT_MASK: c_uint = 1;
const XK_RETURN: c_ulong = 0xff0d;
const XK_D: c_ulong = 0x64;
const XK_Q: c_ulong = 0x71;
const XK_1: c_ulong = 0x31;
const XK_2: c_ulong = 0x32;
const XK_3: c_ulong = 0x33;
const XK_4: c_ulong = 0x34;
const XK_5: c_ulong = 0x35;
const XK_6: c_ulong = 0x36;
const XK_7: c_ulong = 0x37;
const XK_8: c_ulong = 0x38;
const XK_9: c_ulong = 0x39;
const PANEL_H: c_int = 34;
const COLOR_BG: c_ulong = 0x171923;
const COLOR_PANEL: c_ulong = 0x222534;
const COLOR_ACCENT: c_ulong = 0xffb84d;
const COLOR_TEXT: c_ulong = 0xf4f1e8;
const COLOR_DIM: c_ulong = 0xaaaebe;

fn cstring(s: &str) -> CString { CString::new(s).unwrap_or_else(|_| CString::new(" ").unwrap()) }

fn spawn_candidates(candidates: &[&str]) -> bool {
    for candidate in candidates {
        if Command::new(candidate).spawn().is_ok() { return true; }
    }
    false
}

fn launch_terminal() {
    if let Ok(term) = std::env::var("TERMINAL") {
        if Command::new(term).spawn().is_ok() { return; }
    }
    if !spawn_candidates(&["foot", "kitty", "alacritty", "wezterm", "xterm", "uxterm", "urxvt"]) {
        eprintln!("LDE: no terminal found. Install xterm, foot, kitty, or set TERMINAL.");
    }
}

fn launch_files() {
    if !spawn_candidates(&["thunar", "pcmanfm", "nemo", "dolphin"]) {
        eprintln!("LDE: no file manager found. Install one (for example PCManFM).");
    }
}

fn launch_web() {
    if !spawn_candidates(&["firefox", "chromium", "chromium-browser", "google-chrome"])
    {
        eprintln!("LDE: no browser found.");
    }
}

fn launch_menu() {
    if Command::new("rofi").args(["-show", "drun"]).spawn().is_ok() { return; }
    if spawn_candidates(&["dmenu_run", "bemenu-run"]) { return; }
    // Safe fallback: show useful launch hints in a terminal instead of silently failing.
    for term in ["foot", "kitty", "alacritty", "xterm", "urxvt"] {
        if Command::new(term).args(["-e", "sh", "-lc", "printf 'LDE launcher\\n\\nInstall rofi or dmenu for an application menu.\\nHotkeys: Super+Enter terminal, Super+1..9 workspaces.\\n'; exec \"${SHELL:-/bin/sh}\""]).spawn().is_ok() { return; }
    }
    eprintln!("LDE: install rofi or dmenu_run for the application launcher.");
}

unsafe fn draw_panel(display: *mut Display, panel: Window, gc: GC, width: c_int) {
    XSetForeground(display, gc, COLOR_PANEL);
    XFillRectangle(display, panel, gc, 0, 0, width.max(1) as c_uint, PANEL_H as c_uint);
    XSetForeground(display, gc, COLOR_ACCENT);
    XFillRectangle(display, panel, gc, 0, 0, 4, PANEL_H as c_uint);
    XSetForeground(display, gc, COLOR_TEXT);
    let title = cstring("LEMON");
    XDrawString(display, panel, gc, 16, 22, title.as_ptr(), 5);
    XSetForeground(display, gc, COLOR_DIM);
    let buttons = cstring("  Terminal    Files    Web    Apps");
    XDrawString(display, panel, gc, 98, 22, buttons.as_ptr(), 34);
    let clock = chrono_like_clock();
    let clock_c = cstring(&clock);
    XSetForeground(display, gc, COLOR_ACCENT);
    XDrawString(display, panel, gc, (width - 86).max(8), 22, clock_c.as_ptr(), clock.len() as c_int);
    XFlush(display);
}

fn chrono_like_clock() -> String {
    // No time crate: use the system date command when available; panel still works without it.
    Command::new("date").arg("+%H:%M").output().ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "LDE".to_string())
}

unsafe fn keycode(display: *mut Display, keysym: c_ulong) -> c_uint {
    XKeysymToKeycode(display, keysym) as c_uint
}

unsafe fn grab(display: *mut Display, root: Window, keysym: c_ulong, modifiers: c_uint) {
    let code = XKeysymToKeycode(display, keysym);
    if code != 0 { XGrabKey(display, code, modifiers, root, 1, 1, 1); }
}

fn main() {
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        println!("LDE lemon {}", env!("CARGO_PKG_VERSION")); return;
    }
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("LDE — tiny X11 desktop shell\n\nUsage: lemon [--help] [--version]\nStart it from an X11 session or display-manager session entry.\nKeyboard: Super+Enter terminal, Super+D app menu, Super+Q close, Super+Shift+Q quit."); return;
    }
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            eprintln!("LDE: cannot open X display. Start an X11 session and check DISPLAY.");
            std::process::exit(1);
        }
        let screen = XDefaultScreen(display);
        let root = XDefaultRootWindow(display);
        let width = XDisplayWidth(display, screen);
        let height = XDisplayHeight(display, screen);
        XSelectInput(display, root, SUBSTRUCTURE_REDIRECT_MASK | SUBSTRUCTURE_NOTIFY_MASK | KEY_PRESS_MASK);
        XSetWindowBackground(display, root, COLOR_BG);
        XClearWindow(display, root);
        for key in [XK_RETURN, XK_D, XK_Q, XK_1, XK_2, XK_3, XK_4, XK_5, XK_6, XK_7, XK_8, XK_9] {
            grab(display, root, key, MOD4_MASK);
        }
        grab(display, root, XK_Q, MOD4_MASK | SHIFT_MASK);

        let panel = XCreateSimpleWindow(display, root, 0, 0, width as c_uint, PANEL_H as c_uint, 0, COLOR_PANEL, COLOR_PANEL);
        let mut attrs: XSetWindowAttributes = std::mem::zeroed();
        attrs.override_redirect = 1;
        XChangeWindowAttributes(display, panel, CW_OVERRIDE_REDIRECT, &mut attrs);
        XSelectInput(display, panel, EXPOSURE_MASK | BUTTON_PRESS_MASK);
        XMapWindow(display, panel);
        XRaiseWindow(display, panel);
        let gc = XCreateGC(display, panel, 0, ptr::null_mut());
        draw_panel(display, panel, gc, width);
        eprintln!("LDE {} running on X11 (screen {}x{}).", env!("CARGO_PKG_VERSION"), width, height);

        let mut windows: HashSet<Window> = HashSet::new();
        let mut focused: Window = 0;
        let mut next_slot: usize = 0;
        let mut current_workspace: usize = 0;
        let mut workspace_of: std::collections::HashMap<Window, usize> = std::collections::HashMap::new();
        loop {
            // Let the panel clock refresh once a minute while keeping the event loop asleep.
            if XPending(display) == 0 {
                let mut fd = PollFd { fd: XConnectionNumber(display), events: 1, revents: 0 };
                let ready = poll(&mut fd, 1, 60_000);
                if ready == 0 { draw_panel(display, panel, gc, width); continue; }
                if ready < 0 { continue; }
            }
            let mut event: XEvent = std::mem::zeroed();
            XNextEvent(display, &mut event);
            match event.type_ {
                MAP_REQUEST => {
                    let e = event.map_request;
                    if e.window == panel { continue; }
                    let slot = next_slot % 7;
                    next_slot += 1;
                    let margin = 24 + (slot as c_int * 28);
                    XSelectInput(display, e.window, BUTTON_PRESS_MASK);
                    XSetWindowBorderWidth(display, e.window, 2);
                    XSetWindowBorder(display, e.window, COLOR_ACCENT);
                    let client_w = ((width - margin * 2).min(width * 3 / 4)).max(320);
                    let client_h = ((height - PANEL_H - margin * 2).min(height * 3 / 4)).max(240);
                    XMoveResizeWindow(display, e.window, margin, PANEL_H + margin,
                        client_w as c_uint, client_h as c_uint);
                    XMapWindow(display, e.window);
                    windows.insert(e.window);
                    workspace_of.insert(e.window, current_workspace);
                    focused = e.window;
                    XSetInputFocus(display, focused, 1, 0);
                    XRaiseWindow(display, panel);
                }
                CONFIGURE_REQUEST => {
                    let e = event.configure_request;
                    XMoveResizeWindow(display, e.window, e.x, e.y.max(PANEL_H),
                        e.width.max(1) as c_uint, e.height.max(1) as c_uint);
                }
                KEY_PRESS => {
                    let e = event.key;
                    if e.state & MOD4_MASK != 0 {
                        // XKeyEvent carries the keycode; compare against server-resolved keycodes.
                        if e.keycode == keycode(display, XK_RETURN) { launch_terminal(); }
                        else if e.keycode == keycode(display, XK_D) { launch_menu(); }
                        else if e.keycode == keycode(display, XK_Q) && e.state & SHIFT_MASK != 0 {
                            XCloseDisplay(display); return;
                        } else if e.keycode == keycode(display, XK_Q) && focused != 0 {
                            XKillClient(display, focused);
                        } else {
                            let index = [XK_1, XK_2, XK_3, XK_4, XK_5, XK_6, XK_7, XK_8, XK_9]
                                .iter().position(|ks| e.keycode == keycode(display, *ks));
                            if let Some(target) = index {
                                if target != current_workspace {
                                    for win in &windows {
                                        if workspace_of.get(win).copied().unwrap_or(0) == current_workspace { XUnmapWindow(display, *win); }
                                    }
                                    current_workspace = target;
                                    for win in &windows {
                                        if workspace_of.get(win).copied().unwrap_or(0) == current_workspace { XMapWindow(display, *win); }
                                    }
                                    if let Some(win) = windows.iter().find(|&&w| workspace_of.get(&w).copied() == Some(current_workspace)) {
                                        focused = *win; XSetInputFocus(display, focused, 1, 0);
                                    } else {
                                        focused = 0; XSetInputFocus(display, root, 1, 0);
                                    }
                                    XRaiseWindow(display, panel);
                                }
                            }
                        }
                    }
                }
                BUTTON_PRESS => {
                    if event.button.window == panel {
                        match event.button.x {
                            70..=174 => launch_terminal(),
                            175..=250 => launch_files(),
                            251..=310 => launch_web(),
                            311..=420 => launch_menu(),
                            _ => {}
                        }
                        XRaiseWindow(display, panel);
                    } else {
                        focused = event.button.window;
                        XRaiseWindow(display, focused);
                        XSetInputFocus(display, focused, 1, 0);
                    }
                }
                EXPOSE => {
                    if event.expose.window == panel { draw_panel(display, panel, gc, width); }
                }
                DESTROY_NOTIFY => {
                    let win = event.destroy.window;
                    windows.remove(&win); workspace_of.remove(&win);
                    if focused == win {
                        if let Some(next) = windows.iter().find(|&&w| workspace_of.get(&w).copied() == Some(current_workspace)) {
                            focused = *next; XSetInputFocus(display, focused, 1, 0);
                        } else {
                            focused = 0; XSetInputFocus(display, root, 1, 0);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[link(name = "X11")]
extern "C" { fn XUnmapWindow(display: *mut Display, window: Window) -> c_int; }
