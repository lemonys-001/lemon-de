//! Small X11 window manager/shell. Core has no third-party Rust dependencies;
//! optional desktop services are separate user processes configured in ~/.config/lde/config.
mod config;
mod native_apps;
mod terminal_ui;

use config::Config;
use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void, CString};
use std::process::{Command, Stdio};
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
struct XMotionEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    window: Window, root: Window, subwindow: Window, time: c_ulong,
    x: c_int, y: c_int, x_root: c_int, y_root: c_int, state: c_uint,
    is_hint: c_char, same_screen: c_int,
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
union XClientMessageData { b: [c_char; 20], s: [c_short; 10], l: [c_long; 5] }
#[repr(C)] #[derive(Clone, Copy)]
struct XClientMessageEvent {
    type_: c_int, serial: c_ulong, send_event: c_int, display: *mut Display,
    window: Window, message_type: c_ulong, format: c_int, data: XClientMessageData,
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
    motion: XMotionEvent,
    map_request: XMapRequestEvent,
    configure_request: XConfigureRequestEvent,
    expose: XExposeEvent,
    client_message: XClientMessageEvent,
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
    fn XChangeProperty(display: *mut Display, window: Window, property: c_ulong, type_: c_ulong,
        format: c_int, mode: c_int, data: *const u8, nelements: c_int) -> c_int;
    fn XDeleteProperty(display: *mut Display, window: Window, property: c_ulong) -> c_int;
    fn XClearWindow(display: *mut Display, window: Window) -> c_int;
    fn XMapWindow(display: *mut Display, window: Window) -> c_int;
    fn XUnmapWindow(display: *mut Display, window: Window) -> c_int;
    fn XMoveResizeWindow(display: *mut Display, window: Window, x: c_int, y: c_int,
        width: c_uint, height: c_uint) -> c_int;
    fn XRaiseWindow(display: *mut Display, window: Window) -> c_int;
    fn XNextEvent(display: *mut Display, event: *mut XEvent) -> c_int;
    fn XMaskEvent(display: *mut Display, mask: c_long, event: *mut XEvent) -> c_int;
    fn XPending(display: *mut Display) -> c_int;
    fn XConnectionNumber(display: *mut Display) -> c_int;
    fn poll(fds: *mut PollFd, count: c_ulong, timeout_ms: c_int) -> c_int;
    fn XCreateGC(display: *mut Display, drawable: Window, mask: c_ulong,
        values: *mut c_void) -> GC;
    fn XSetForeground(display: *mut Display, gc: GC, foreground: c_ulong) -> c_int;
    fn XFillRectangle(display: *mut Display, drawable: Window, gc: GC, x: c_int,
        y: c_int, width: c_uint, height: c_uint) -> c_int;
    fn XFillArc(display: *mut Display, drawable: Window, gc: GC, x: c_int,
        y: c_int, width: c_uint, height: c_uint, angle1: c_int, angle2: c_int) -> c_int;
    fn XDrawString(display: *mut Display, drawable: Window, gc: GC, x: c_int,
        y: c_int, string: *const c_char, length: c_int) -> c_int;
    fn XFlush(display: *mut Display) -> c_int;
    fn XSetInputFocus(display: *mut Display, window: Window, revert_to: c_int,
        time: c_ulong) -> c_int;
    fn XSetWindowBorderWidth(display: *mut Display, window: Window, width: c_uint) -> c_int;
    fn XSetWindowBorder(display: *mut Display, window: Window, pixel: c_ulong) -> c_int;
    fn XInternAtom(display: *mut Display, name: *const c_char, only_if_exists: c_int) -> c_ulong;
    fn XGetWMProtocols(display: *mut Display, window: Window, protocols: *mut *mut c_ulong, count: *mut c_int) -> c_int;
    fn XSendEvent(display: *mut Display, window: Window, propagate: c_int, event_mask: c_long, event: *mut XEvent) -> c_int;
    fn XFree(data: *mut c_void) -> c_int;
    fn XKillClient(display: *mut Display, resource: c_ulong) -> c_int;
    fn XGrabKey(display: *mut Display, keycode: c_int, modifiers: c_uint,
        grab_window: Window, owner_events: c_int, pointer_mode: c_int,
        keyboard_mode: c_int) -> c_int;
    fn XKeysymToKeycode(display: *mut Display, keysym: c_ulong) -> c_int;
    fn XGetGeometry(display: *mut Display, drawable: Window, root: *mut Window,
        x: *mut c_int, y: *mut c_int, width: *mut c_uint, height: *mut c_uint,
        border_width: *mut c_uint, depth: *mut c_uint) -> c_int;
    fn XGrabButton(display: *mut Display, button: c_uint, modifiers: c_uint,
        grab_window: Window, owner_events: c_int, event_mask: c_uint,
        pointer_mode: c_int, keyboard_mode: c_int, confine_to: Window, cursor: c_ulong) -> c_int;
    fn XUngrabPointer(display: *mut Display, time: c_ulong) -> c_int;
    fn XAllowEvents(display: *mut Display, mode: c_int, time: c_ulong) -> c_int;
}

extern "C" { fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int; }
const WNOHANG: c_int = 1;
unsafe fn reap_children() {
    loop {
        let mut status = 0;
        if waitpid(-1, &mut status, WNOHANG) <= 0 { break; }
    }
}

const KEY_PRESS: c_int = 2;
const BUTTON_PRESS: c_int = 4;
const BUTTON_RELEASE: c_int = 5;
const MOTION_NOTIFY: c_int = 6;
const EXPOSE: c_int = 12;
const DESTROY_NOTIFY: c_int = 17;
const CLIENT_MESSAGE: c_int = 33;
const MAP_REQUEST: c_int = 20;
const CONFIGURE_REQUEST: c_int = 23;
const KEY_PRESS_MASK: c_long = 1;
const BUTTON_PRESS_MASK: c_long = 1 << 2;
const BUTTON_RELEASE_MASK: c_long = 1 << 3;
const POINTER_MOTION_MASK: c_long = 1 << 6;
const EXPOSURE_MASK: c_long = 1 << 15;
const SUBSTRUCTURE_NOTIFY_MASK: c_long = 1 << 19;
const SUBSTRUCTURE_REDIRECT_MASK: c_long = 1 << 20;
const CW_OVERRIDE_REDIRECT: c_ulong = 1 << 9;
const MOD1_MASK: c_uint = 1 << 3;
const ANY_MODIFIER: c_uint = 1 << 15;
const ASYNC_POINTER: c_int = 0;
const REPLAY_POINTER: c_int = 2;
const MOD4_MASK: c_uint = 1 << 6;
const SHIFT_MASK: c_uint = 1;
const XK_RETURN: c_ulong = 0xff0d;
const XK_D: c_ulong = 0x64;
const XK_I: c_ulong = 0x69;
const XK_O: c_ulong = 0x6f;
const XK_Q: c_ulong = 0x71;
const XK_T: c_ulong = 0x74;
const XK_M: c_ulong = 0x6d;
const XK_C: c_ulong = 0x63;
const XK_TAB: c_ulong = 0xff09;
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
const COLOR_TEXT: c_ulong = 0xf4f1e8;
const COLOR_DIM: c_ulong = 0xaaaebe;
const COLOR_FOCUS: c_ulong = 0x60a5fa;
const COLOR_UNFOCUSED: c_ulong = 0x505566;

#[derive(Clone, Copy)]
struct Geometry { x: c_int, y: c_int, width: c_uint, height: c_uint }
#[derive(Clone, Copy)]
struct Ewmh { current_desktop: c_ulong, window_desktop: c_ulong, active_window: c_ulong, close_window: c_ulong }

fn cstring(s: &str) -> CString { CString::new(s).unwrap_or_else(|_| CString::new(" ").unwrap()) }

const XA_ATOM: c_ulong = 4;
const XA_CARDINAL: c_ulong = 6;
const XA_WINDOW: c_ulong = 33;
unsafe fn intern_atom(display: *mut Display, name: &str) -> c_ulong {
    XInternAtom(display, cstring(name).as_ptr(), 0)
}
unsafe fn change_u32_property(display: *mut Display, window: Window, property: c_ulong,
                              type_: c_ulong, values: &[c_ulong]) {
    XChangeProperty(display, window, property, type_, 32, 0,
        values.as_ptr().cast::<u8>(), values.len() as c_int);
}
unsafe fn init_ewmh(display: *mut Display, root: Window) -> Ewmh {
    let names = ["_NET_SUPPORTED", "_NET_SUPPORTING_WM_CHECK", "_NET_WM_NAME",
        "_NET_ACTIVE_WINDOW", "_NET_NUMBER_OF_DESKTOPS", "_NET_CURRENT_DESKTOP",
        "_NET_DESKTOP_NAMES", "_NET_WM_DESKTOP", "_NET_CLOSE_WINDOW"];
    let atoms: Vec<c_ulong> = names.iter().map(|name| intern_atom(display, name)).collect();
    let check = XCreateSimpleWindow(display, root, -1, -1, 1, 1, 0, 0, 0);
    let support = atoms[1];
    let utf8 = intern_atom(display, "UTF8_STRING");
    change_u32_property(display, root, support, XA_WINDOW, &[check]);
    change_u32_property(display, check, support, XA_WINDOW, &[check]);
    let wm_name = cstring("LDE");
    XChangeProperty(display, check, atoms[2], utf8, 8, 0,
        wm_name.as_bytes().as_ptr(), wm_name.as_bytes().len() as c_int);
    change_u32_property(display, root, atoms[0], XA_ATOM, &atoms[1..]);
    change_u32_property(display, root, atoms[4], XA_CARDINAL, &[9]);
    change_u32_property(display, root, atoms[5], XA_CARDINAL, &[0]);
    let mut desktop_names = Vec::new();
    for index in 1..=9 { desktop_names.extend_from_slice(format!("Workspace {index}\0").as_bytes()); }
    XChangeProperty(display, root, atoms[6], utf8, 8, 0,
        desktop_names.as_ptr(), desktop_names.len() as c_int);
    Ewmh { current_desktop: atoms[5], window_desktop: atoms[7], active_window: atoms[3], close_window: atoms[8] }
}
unsafe fn set_ewmh_desktop(display: *mut Display, ewmh: &Ewmh, window: Window, desktop: usize) {
    change_u32_property(display, window, ewmh.window_desktop, XA_CARDINAL, &[desktop as c_ulong]);
}
unsafe fn set_ewmh_current_desktop(display: *mut Display, root: Window, ewmh: &Ewmh, desktop: usize) {
    change_u32_property(display, root, ewmh.current_desktop, XA_CARDINAL, &[desktop as c_ulong]);
}
unsafe fn set_ewmh_active_window(display: *mut Display, root: Window, ewmh: &Ewmh, window: Window) {
    if window == 0 { XDeleteProperty(display, root, ewmh.active_window); }
    else { change_u32_property(display, root, ewmh.active_window, XA_WINDOW, &[window]); }
}
fn spawn_background(command: &mut Command) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    command.spawn().is_ok()
}
fn spawn_named(program: &str, args: &[&str]) -> bool {
    let mut child = Command::new(program);
    child.args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    spawn_background(&mut child)
}
fn shell_command(command: &str) -> bool {
    if command.trim().is_empty() { return false; }
    let mut child = Command::new("sh");
    child.arg("-lc").arg(command).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    spawn_background(&mut child)
}
fn spawn_candidates(candidates: &[&str]) -> bool {
    for candidate in candidates {
        let mut child = Command::new(candidate);
        child.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        if spawn_background(&mut child) { return true; }
    }
    false
}

fn launch_terminal(config: &Config) {
    if let Ok(term) = std::env::var("TERMINAL") {
        if shell_command(&term) { return; }
    }
    if !config.terminal.is_empty() && shell_command(&config.terminal) { return; }
    if !spawn_candidates(&["foot", "kitty", "alacritty", "wezterm", "xterm", "uxterm", "urxvt"]) {
        eprintln!("LDE: no terminal found. Install xterm, foot, kitty, or set terminal in the LDE config.");
    }
}
fn spawn_builtin(option: &str) -> bool {
    let Ok(exe) = std::env::current_exe() else { return false; };
    let mut child = Command::new(exe);
    child.arg(option).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    spawn_background(&mut child)
}
fn launch_files(config: &Config) {
    if !config.file_manager.is_empty() && shell_command(&config.file_manager) { return; }
    if std::env::var_os("DISPLAY").is_some() && spawn_builtin("--files") { return; }
    if spawn_candidates(&["thunar", "pcmanfm", "nemo", "dolphin"]) { return; }
    eprintln!("LDE: no file manager found; use the built-in Lemon Files app or install one.");
}
fn launch_calculator() {
    if !spawn_builtin("--calculator") { eprintln!("LDE: could not launch Lemon Calculator."); }
}
fn launch_web(config: &Config) {
    if (!config.browser.is_empty() && shell_command(&config.browser))
        || spawn_candidates(&["firefox", "chromium", "chromium-browser", "google-chrome"]) { return; }
    eprintln!("LDE: no browser found; install one or set browser in the LDE config.");
}
fn launch_menu(config: &Config) {
    if !config.launcher.is_empty() && shell_command(&config.launcher) { return; }
    if std::env::var_os("DISPLAY").is_some() && spawn_builtin("--launcher") { return; }
    if spawn_named("rofi", &["-show", "drun"]) || spawn_candidates(&["dmenu_run", "bemenu-run"]) { return; }
    let notice = "printf 'LDE launcher\\n\\nInstall rofi or dmenu for an application menu.\\nHotkeys: Super+Enter terminal, Super+1..9 workspaces.\\n'; exec \"${SHELL:-/bin/sh}\"";
    for term in ["foot", "kitty", "alacritty", "xterm", "urxvt"] {
        if spawn_named(term, &["-e", "sh", "-lc", notice]) { return; }
    }
    eprintln!("LDE: install rofi or dmenu_run for the application launcher.");
}
unsafe fn request_close(display: *mut Display, window: Window) {
    let protocols_atom = XInternAtom(display, cstring("WM_PROTOCOLS").as_ptr(), 0);
    let delete_atom = XInternAtom(display, cstring("WM_DELETE_WINDOW").as_ptr(), 0);
    let mut protocols: *mut c_ulong = ptr::null_mut();
    let mut count = 0;
    if XGetWMProtocols(display, window, &mut protocols, &mut count) != 0 {
        let supports_delete = !protocols.is_null()
            && std::slice::from_raw_parts(protocols, count.max(0) as usize).contains(&delete_atom);
        if !protocols.is_null() { XFree(protocols.cast()); }
        if supports_delete {
            let mut event: XEvent = std::mem::zeroed();
            event.client_message = XClientMessageEvent {
                type_: CLIENT_MESSAGE, serial: 0, send_event: 1, display, window,
                message_type: protocols_atom, format: 32,
                data: XClientMessageData { l: [delete_atom as c_long, 0, 0, 0, 0] },
            };
            XSendEvent(display, window, 0, 0, &mut event);
            XFlush(display);
            return;
        }
    }
    XKillClient(display, window);
}

fn shell_quote(value: &str) -> String { format!("'{}'", value.replace('\'', "'\\''")) }
fn open_control_center() {
    if std::env::var_os("DISPLAY").is_some() && spawn_builtin("--control-center") { return; }
    eprintln!("LDE: the graphical Control Center requires an X11 display.");
}
fn open_settings() {
    if std::env::var_os("DISPLAY").is_some() && spawn_builtin("--settings") { return; }
    let path = match Config::ensure_file() {
        Ok(path) => path,
        Err(error) => { eprintln!("LDE: {error}"); return; }
    };
    let path_arg = path.to_string_lossy().to_string();
    for editor in ["mousepad", "featherpad", "xed", "geany", "leafpad", "gedit"] {
        if spawn_named(editor, &[path_arg.as_str()]) { return; }
    }
    let command = format!("${{EDITOR:-vi}} {}", shell_quote(&path_arg));
    let mut terminals: Vec<String> = std::env::var("TERMINAL").ok().into_iter().collect();
    terminals.extend(["foot", "kitty", "alacritty", "xterm", "urxvt"].into_iter().map(str::to_owned));
    for term in terminals {
        if spawn_named(&term, &["-e", "sh", "-lc", command.as_str()]) { return; }
    }
    eprintln!("LDE settings file: {} (set EDITOR or install a text editor and terminal).", path.display());
}
fn start_optional_services(config: &Config) {
    if !config.wallpaper.is_empty() {
        let path = &config.wallpaper;
        let feh = Command::new("feh").args(["--bg-fill", path]).stdin(Stdio::null()).status();
        if !matches!(feh, Ok(status) if status.success()) {
            let nitrogen = Command::new("nitrogen").args(["--set-zoom-fill", path]).stdin(Stdio::null()).status();
            if !matches!(nitrogen, Ok(status) if status.success()) {
                eprintln!("LDE: wallpaper configured, but neither feh nor nitrogen could apply it.");
            }
        }
    }
    for command in &config.autostart {
        if !shell_command(command) { eprintln!("LDE: could not start configured service: {command}"); }
    }
}

unsafe fn panel_pill(display: *mut Display, panel: Window, gc: GC, x: c_int,
                     y: c_int, width: c_uint, height: c_uint, color: c_ulong) {
    let radius = 7u32;
    let diameter = radius * 2;
    XSetForeground(display, gc, color);
    XFillRectangle(display, panel, gc, x + radius as c_int, y,
        width.saturating_sub(diameter), height);
    XFillRectangle(display, panel, gc, x, y + radius as c_int,
        width, height.saturating_sub(diameter));
    for (cx, cy) in [(x, y), (x + width as c_int - diameter as c_int, y),
        (x, y + height as c_int - diameter as c_int),
        (x + width as c_int - diameter as c_int, y + height as c_int - diameter as c_int)] {
        XFillArc(display, panel, gc, cx, cy, diameter, diameter, 0, 360 * 64);
    }
}

unsafe fn draw_panel(display: *mut Display, panel: Window, gc: GC, width: c_int,
                     config: &Config, current: usize, tiled: bool) {
    XSetForeground(display, gc, config.panel_color as c_ulong);
    XFillRectangle(display, panel, gc, 0, 0, width.max(1) as c_uint, PANEL_H as c_uint);
    XSetForeground(display, gc, config.accent_color as c_ulong);
    XFillRectangle(display, panel, gc, 0, PANEL_H - 2, width.max(1) as c_uint, 2);
    XSetForeground(display, gc, COLOR_TEXT);
    let title = cstring("LDE");
    XDrawString(display, panel, gc, 18, 22, title.as_ptr(), 3);
    XSetForeground(display, gc, COLOR_UNFOCUSED);
    XFillRectangle(display, panel, gc, 76, 8, 1, 18);

    for (label, x, w) in [("Terminal", 90, 73), ("Files", 164, 55), ("Web", 220, 55), ("Calc", 276, 50)] {
        panel_pill(display, panel, gc, x, 4, w, 26, 0x2b3548);
        XSetForeground(display, gc, COLOR_TEXT);
        let text = cstring(label);
        XDrawString(display, panel, gc, x + 9, 22, text.as_ptr(), label.len() as c_int);
    }
    panel_pill(display, panel, gc, 330, 4, 62, 26, config.accent_color as c_ulong);
    XSetForeground(display, gc, config.panel_color as c_ulong);
    let apps = cstring("Apps");
    XDrawString(display, panel, gc, 341, 22, apps.as_ptr(), 4);
    panel_pill(display, panel, gc, 397, 4, 86, 26, 0x2b3548);
    XSetForeground(display, gc, COLOR_TEXT);
    let settings = cstring("Settings");
    XDrawString(display, panel, gc, 406, 22, settings.as_ptr(), 8);

    if width >= 900 {
        for i in 0..9 {
            let x = 510 + i as c_int * 24;
            if i == current {
                panel_pill(display, panel, gc, x - 7, 4, 22, 26, config.accent_color as c_ulong);
                XSetForeground(display, gc, config.panel_color as c_ulong);
            } else { XSetForeground(display, gc, COLOR_DIM); }
            let label = cstring(&(i + 1).to_string());
            XDrawString(display, panel, gc, x, 22, label.as_ptr(), 1);
        }
    }
    if width >= 720 {
        let mode = cstring(if tiled { "TILE" } else { "FLOAT" });
        XSetForeground(display, gc, config.accent_color as c_ulong);
        XDrawString(display, panel, gc, (width - 150).max(8), 22, mode.as_ptr(), 4 + if tiled { 0 } else { 1 });
    }
    if width >= 620 {
        let clock = chrono_like_clock();
        let clock_c = cstring(&clock);
        XSetForeground(display, gc, COLOR_TEXT);
        XDrawString(display, panel, gc, (width - 76).max(8), 22, clock_c.as_ptr(), clock.len() as c_int);
    }
    XFlush(display);
}
fn chrono_like_clock() -> String {
    Command::new("date").arg("+%H:%M").output().ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "LDE".to_string())
}
unsafe fn keycode(display: *mut Display, keysym: c_ulong) -> c_uint { XKeysymToKeycode(display, keysym) as c_uint }
unsafe fn grab(display: *mut Display, root: Window, keysym: c_ulong, modifiers: c_uint) {
    let code = XKeysymToKeycode(display, keysym);
    if code != 0 { XGrabKey(display, code, modifiers, root, 1, 1, 1); }
}
unsafe fn geometry(display: *mut Display, window: Window) -> Option<Geometry> {
    let mut root = 0; let mut x = 0; let mut y = 0;
    let mut width = 0; let mut height = 0; let mut border = 0; let mut depth = 0;
    if XGetGeometry(display, window, &mut root, &mut x, &mut y, &mut width, &mut height, &mut border, &mut depth) != 0 {
        Some(Geometry { x, y, width, height })
    } else { None }
}
unsafe fn arrange_tiled(display: *mut Display, windows: &HashSet<Window>,
                        workspace_of: &HashMap<Window, usize>, current: usize,
                        width: c_int, height: c_int, panel_top: bool) {
    let mut active: Vec<Window> = windows.iter().copied()
        .filter(|w| workspace_of.get(w).copied() == Some(current)).collect();
    active.sort_unstable();
    if active.is_empty() { return; }
    let area_y = if panel_top { PANEL_H } else { 0 };
    let area_h = (height - PANEL_H).max(1);
    if active.len() == 1 {
        XMoveResizeWindow(display, active[0], 0, area_y, width.max(1) as c_uint, area_h as c_uint);
        return;
    }
    let master_w = (width * 3 / 5).max(240).min(width - 160).max(1);
    let stack_x = master_w;
    let stack_w = (width - master_w).max(1);
    XMoveResizeWindow(display, active[0], 0, area_y, master_w as c_uint, area_h as c_uint);
    let stack_count = (active.len() - 1) as c_int;
    let slot_h = (area_h / stack_count).max(1);
    for (index, window) in active.iter().skip(1).enumerate() {
        let i = index as c_int;
        let y = area_y + i * slot_h;
        let h = if i == stack_count - 1 { area_y + area_h - y } else { slot_h };
        XMoveResizeWindow(display, *window, stack_x, y, stack_w as c_uint, h.max(1) as c_uint);
    }
}
unsafe fn restore_floating(display: *mut Display, windows: &HashSet<Window>,
                           workspace_of: &HashMap<Window, usize>, current: usize,
                           saved: &mut HashMap<Window, Geometry>, width: c_int, height: c_int,
                           panel_top: bool) {
    for window in windows {
        if workspace_of.get(window).copied() != Some(current) { continue; }
        let g = saved.remove(window).unwrap_or(Geometry {
            x: 24, y: if panel_top { PANEL_H + 24 } else { 24 },
            width: (width * 3 / 4).max(320) as c_uint,
            height: ((height - PANEL_H) * 3 / 4).max(240) as c_uint,
        });
        XMoveResizeWindow(display, *window, g.x, g.y, g.width.max(1), g.height.max(1));
    }
}
unsafe fn toggle_layout(display: *mut Display, windows: &HashSet<Window>,
                        workspace_of: &HashMap<Window, usize>, current: usize,
                        tiled: &mut [bool; 9], saved: &mut HashMap<Window, Geometry>,
                        width: c_int, height: c_int, panel_top: bool) {
    if tiled[current] {
        restore_floating(display, windows, workspace_of, current, saved, width, height, panel_top);
        tiled[current] = false;
    } else {
        for window in windows {
            if workspace_of.get(window).copied() == Some(current) {
                if let Some(g) = geometry(display, *window) { saved.insert(*window, g); }
            }
        }
        tiled[current] = true;
        arrange_tiled(display, windows, workspace_of, current, width, height, panel_top);
    }
}
unsafe fn switch_workspace(display: *mut Display, root: Window, panel: Window, gc: GC,
                           ewmh: &Ewmh, config: &Config, windows: &HashSet<Window>,
                           workspace_of: &HashMap<Window, usize>, current: &mut usize,
                           target: usize, tiled: &[bool; 9], width: c_int, height: c_int,
                           panel_top: bool, focused: &mut Window) {
    if target >= 9 || target == *current { return; }
    let old = *current;
    for window in windows {
        if workspace_of.get(window).copied() == Some(old) { XUnmapWindow(display, *window); }
    }
    *current = target;
    set_ewmh_current_desktop(display, root, ewmh, target);
    for window in windows {
        if workspace_of.get(window).copied() == Some(target) { XMapWindow(display, *window); }
    }
    if tiled[target] { arrange_tiled(display, windows, workspace_of, target, width, height, panel_top); }
    if let Some(window) = windows.iter().find(|&&w| workspace_of.get(&w).copied() == Some(target)) {
        *focused = *window;
        XSetInputFocus(display, *focused, 1, 0);
        XRaiseWindow(display, *focused);
    } else {
        *focused = 0;
        XSetInputFocus(display, root, 1, 0);
    }
    set_ewmh_active_window(display, root, ewmh, *focused);
    XRaiseWindow(display, panel);
    draw_panel(display, panel, gc, width, config, target, tiled[target]);
}
unsafe fn drag_window(display: *mut Display, panel: Window, window: Window,
                      button: c_uint, start_x: c_int, start_y: c_int) {
    let Some(initial) = geometry(display, window) else { return; };
    let event_mask = POINTER_MOTION_MASK | BUTTON_RELEASE_MASK;
    loop {
        let mut event: XEvent = std::mem::zeroed();
        XMaskEvent(display, event_mask, &mut event);
        match event.type_ {
            MOTION_NOTIFY => {
                let motion = event.motion;
                let dx = motion.x_root - start_x;
                let dy = motion.y_root - start_y;
                if button == 1 {
                    XMoveResizeWindow(display, window, initial.x + dx, initial.y + dy,
                        initial.width, initial.height);
                } else {
                    XMoveResizeWindow(display, window, initial.x, initial.y,
                        (initial.width as c_int + dx).max(240) as c_uint,
                        (initial.height as c_int + dy).max(160) as c_uint);
                }
            }
            BUTTON_RELEASE => break,
            _ => {}
        }
    }
    XUngrabPointer(display, 0);
    XRaiseWindow(display, panel);
}

fn should_run_terminal_menu(args: &[String], stdin_is_tty: bool, display_is_set: bool) -> bool {
    args.iter().any(|arg| arg == "--menu")
        || (args.len() == 1 && stdin_is_tty && !display_is_set)
}

#[cfg(test)]
mod startup_tests {
    use super::should_run_terminal_menu;

    #[test]
    fn graphical_display_takes_precedence_over_terminal_menu() {
        let args = vec!["lemon".to_owned()];
        assert!(!should_run_terminal_menu(&args, true, true));
        assert!(should_run_terminal_menu(&args, true, false));
        assert!(!should_run_terminal_menu(&args, false, false));
    }

    #[test]
    fn explicit_menu_flag_works_even_with_a_display() {
        let args = vec!["lemon".to_owned(), "--menu".to_owned()];
        assert!(should_run_terminal_menu(&args, false, true));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("LDE lemon {}", env!("CARGO_PKG_VERSION")); return;
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("LDE — lightweight X11 desktop shell and tools\n\nUsage: lemon [--menu|--no-menu|--settings|--control-center|--install|--launcher|--files [PATH]|--calculator]\n       lemon --help | --version | --config-path | --init-config\n\nWith no arguments in a terminal without DISPLAY, Lemon opens its interactive menu. If DISPLAY is set (including Termux/VNC), it starts the X11 desktop; use --menu to force the terminal menu.\nShortcuts: Super+Enter terminal, Super+D launcher, Super+I settings, Super+O control center, Super+C calculator, Super+M minimize, Super+Shift+M restore, Alt+Tab cycle, Super+T layout, Super+Q close, Super+Shift+Q quit, Super+1..9 workspaces. Alt+left/right-drag moves/resizes."); return;
    }
    if args.iter().any(|a| a == "--config-path") { println!("{}", Config::path().display()); return; }
    if args.iter().any(|a| a == "--init-config") {
        match Config::ensure_file() { Ok(path) => println!("LDE config ready: {}", path.display()), Err(e) => { eprintln!("LDE: {e}"); std::process::exit(1); } }
        return;
    }
    let display_is_set = std::env::var_os("DISPLAY").is_some();
    if args.iter().any(|a| a == "--control-center") {
        if !display_is_set || !native_apps::control_center() { terminal_ui::run_menu(); }
        return;
    }
    if args.iter().any(|a| a == "--settings") {
        if !display_is_set || !native_apps::settings() { terminal_ui::settings(); }
        return;
    }
    if args.iter().any(|a| a == "--install") { terminal_ui::install_components(); return; }
    if args.iter().any(|a| a == "--launcher") {
        if !display_is_set || !native_apps::launcher() { terminal_ui::launcher(); }
        return;
    }
    if args.iter().any(|a| a == "--files") {
        if !display_is_set || !native_apps::file_browser() { terminal_ui::file_browser(); }
        return;
    }
    if args.iter().any(|a| a == "--calculator") {
        if !display_is_set || !native_apps::calculator() { terminal_ui::calculator(); }
        return;
    }
    if should_run_terminal_menu(&args, terminal_ui::stdin_is_terminal(), display_is_set) {
        terminal_ui::run_menu(); return;
    }
    start_desktop();
}

pub(crate) fn start_desktop() {
    let config = Config::load();
    let panel_top = config.panel_position != "bottom";
    let mut tiled = [config.layout == "tiled"; 9];
    start_optional_services(&config);
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            eprintln!("LDE: cannot open X display. Start an X11 session and check DISPLAY.");
            std::process::exit(1);
        }
        let screen = XDefaultScreen(display);
        let root = XDefaultRootWindow(display);
        let ewmh = init_ewmh(display, root);
        let width = XDisplayWidth(display, screen);
        let height = XDisplayHeight(display, screen);
        XSelectInput(display, root, SUBSTRUCTURE_REDIRECT_MASK | SUBSTRUCTURE_NOTIFY_MASK | KEY_PRESS_MASK);
        if config.wallpaper.is_empty() { XSetWindowBackground(display, root, config.background_color as c_ulong); }
        XClearWindow(display, root);
        for key in [XK_RETURN, XK_D, XK_I, XK_O, XK_Q, XK_T, XK_M, XK_C, XK_1, XK_2, XK_3, XK_4, XK_5, XK_6, XK_7, XK_8, XK_9] {
            grab(display, root, key, MOD4_MASK);
        }
        grab(display, root, XK_Q, MOD4_MASK | SHIFT_MASK);
        grab(display, root, XK_M, MOD4_MASK | SHIFT_MASK);
        grab(display, root, XK_TAB, MOD1_MASK);

        let panel_y = if panel_top { 0 } else { height - PANEL_H };
        let panel = XCreateSimpleWindow(display, root, 0, panel_y, width as c_uint, PANEL_H as c_uint, 0,
            config.panel_color as c_ulong, config.panel_color as c_ulong);
        let mut attrs: XSetWindowAttributes = std::mem::zeroed();
        attrs.override_redirect = 1;
        XChangeWindowAttributes(display, panel, CW_OVERRIDE_REDIRECT, &mut attrs);
        XSelectInput(display, panel, EXPOSURE_MASK | BUTTON_PRESS_MASK);
        XMapWindow(display, panel);
        XRaiseWindow(display, panel);
        let gc = XCreateGC(display, panel, 0, ptr::null_mut());
        draw_panel(display, panel, gc, width, &config, 0, tiled[0]);
        eprintln!("LDE {} on X11 {}x{}; config {}", env!("CARGO_PKG_VERSION"), width, height, Config::path().display());

        let mut windows: HashSet<Window> = HashSet::new();
        let mut focused: Window = 0;
        let mut next_slot: usize = 0;
        let mut current_workspace: usize = 0;
        let mut workspace_of: HashMap<Window, usize> = HashMap::new();
        let mut saved_geometry: HashMap<Window, Geometry> = HashMap::new();
        let mut minimized: Vec<Window> = Vec::new();
        loop {
            reap_children();
            if XPending(display) == 0 {
                let mut fd = PollFd { fd: XConnectionNumber(display), events: 1, revents: 0 };
                let ready = poll(&mut fd, 1, 60_000);
                if ready == 0 { draw_panel(display, panel, gc, width, &config, current_workspace, tiled[current_workspace]); continue; }
                if ready < 0 { continue; }
            }
            let mut event: XEvent = std::mem::zeroed();
            XNextEvent(display, &mut event);
            match event.type_ {
                MAP_REQUEST => {
                    let e = event.map_request;
                    if e.window == panel { continue; }
                    if windows.contains(&e.window) || minimized.contains(&e.window) {
                        minimized.retain(|window| *window != e.window);
                        windows.insert(e.window);
                        workspace_of.insert(e.window, current_workspace);
                        set_ewmh_desktop(display, &ewmh, e.window, current_workspace);
                        XMapWindow(display, e.window);
                        XRaiseWindow(display, e.window);
                        focused = e.window;
                        set_ewmh_active_window(display, root, &ewmh, focused);
                        XSetInputFocus(display, focused, 1, 0);
                        if tiled[current_workspace] { arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top); }
                        XRaiseWindow(display, panel);
                        continue;
                    }
                    let slot = next_slot % 7;
                    next_slot += 1;
                    let area_y = if panel_top { PANEL_H } else { 0 };
                    let usable_h = (height - PANEL_H).max(1);
                    let client_w = width.min(720).max(1) as c_uint;
                    let client_h = usable_h.min(560).max(1) as c_uint;
                    let cascade = slot as c_int * 24;
                    let client_x = ((width - client_w as c_int) / 2 + cascade)
                        .min(width - client_w as c_int).max(0);
                    let client_y = area_y + ((usable_h - client_h as c_int) / 2 + cascade)
                        .min(usable_h - client_h as c_int).max(0);
                    let initial = Geometry { x: client_x, y: client_y, width: client_w, height: client_h };
                    let drag_mask = (BUTTON_PRESS_MASK | POINTER_MOTION_MASK | BUTTON_RELEASE_MASK) as c_uint;
                    // Observe client clicks passively; replay ordinary clicks and retain grabs only for Alt-drag.
                    XGrabButton(display, 1, ANY_MODIFIER, e.window, 0, drag_mask, 0, 1, 0, 0);
                    XGrabButton(display, 3, ANY_MODIFIER, e.window, 0, drag_mask, 0, 1, 0, 0);
                    XSetWindowBorderWidth(display, e.window, 2);
                    XSetWindowBorder(display, e.window, config.accent_color as c_ulong);
                    XMoveResizeWindow(display, e.window, initial.x, initial.y, initial.width, initial.height);
                    XMapWindow(display, e.window);
                    windows.insert(e.window);
                    workspace_of.insert(e.window, current_workspace);
                    set_ewmh_desktop(display, &ewmh, e.window, current_workspace);
                    saved_geometry.insert(e.window, initial);
                    focused = e.window;
                    XSetInputFocus(display, focused, 1, 0);
                    if tiled[current_workspace] {
                        saved_geometry.insert(e.window, initial);
                        arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top);
                    }
                    XRaiseWindow(display, panel);
                }
                CONFIGURE_REQUEST => {
                    let e = event.configure_request;
                    let y = if panel_top { e.y.max(PANEL_H) } else { e.y.min(height - PANEL_H) };
                    XMoveResizeWindow(display, e.window, e.x, y, e.width.max(1) as c_uint, e.height.max(1) as c_uint);
                    if !tiled[current_workspace] && workspace_of.get(&e.window).copied() == Some(current_workspace) {
                        saved_geometry.insert(e.window, Geometry { x: e.x, y, width: e.width.max(1) as c_uint, height: e.height.max(1) as c_uint });
                    }
                    if tiled[current_workspace] { arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top); }
                }
                KEY_PRESS => {
                    let e = event.key;
                    if e.state & MOD1_MASK != 0 && e.keycode == keycode(display, XK_TAB) {
                        let mut visible: Vec<Window> = windows.iter().copied()
                            .filter(|window| workspace_of.get(window).copied() == Some(current_workspace)).collect();
                        visible.sort_unstable();
                        if !visible.is_empty() {
                            let next = visible.iter().position(|window| *window == focused)
                                .map(|index| (index + 1) % visible.len()).unwrap_or(0);
                            if focused != 0 { XSetWindowBorder(display, focused, COLOR_UNFOCUSED); }
                            focused = visible[next];
                            set_ewmh_active_window(display, root, &ewmh, focused);
                            XRaiseWindow(display, focused);
                            XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                            XSetInputFocus(display, focused, 1, 0);
                            XRaiseWindow(display, panel);
                        }
                        continue;
                    }
                    if e.state & MOD4_MASK == 0 { continue; }
                    if e.keycode == keycode(display, XK_RETURN) { launch_terminal(&config); }
                    else if e.keycode == keycode(display, XK_D) { launch_menu(&config); }
                    else if e.keycode == keycode(display, XK_I) { open_settings(); }
                    else if e.keycode == keycode(display, XK_O) { open_control_center(); }
                    else if e.keycode == keycode(display, XK_C) { launch_calculator(); }
                    else if e.keycode == keycode(display, XK_Q) && e.state & SHIFT_MASK != 0 {
                        XCloseDisplay(display); return;
                    } else if e.keycode == keycode(display, XK_Q) && focused != 0 {
                        request_close(display, focused);
                    } else if e.keycode == keycode(display, XK_M) && e.state & SHIFT_MASK != 0 {
                        while let Some(window) = minimized.pop() {
                            if workspace_of.contains_key(&window) {
                                windows.insert(window);
                                workspace_of.insert(window, current_workspace);
                                set_ewmh_desktop(display, &ewmh, window, current_workspace);
                                XMapWindow(display, window);
                                XRaiseWindow(display, window);
                                focused = window;
                                set_ewmh_active_window(display, root, &ewmh, focused);
                                XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                                XSetInputFocus(display, focused, 1, 0);
                                if tiled[current_workspace] { arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top); }
                                XRaiseWindow(display, panel);
                                break;
                            }
                        }
                    } else if e.keycode == keycode(display, XK_M) && focused != 0 {
                        let hidden = focused;
                        XUnmapWindow(display, hidden);
                        windows.remove(&hidden);
                        minimized.retain(|window| *window != hidden);
                        minimized.push(hidden);
                        if let Some(next) = windows.iter().copied().find(|window| workspace_of.get(window).copied() == Some(current_workspace)) {
                            focused = next;
                            set_ewmh_active_window(display, root, &ewmh, focused);
                            XRaiseWindow(display, focused);
                            XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                            XSetInputFocus(display, focused, 1, 0);
                        } else {
                            focused = 0;
                            set_ewmh_active_window(display, root, &ewmh, focused);
                            XSetInputFocus(display, root, 1, 0);
                        }
                        if tiled[current_workspace] { arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top); }
                    } else if e.keycode == keycode(display, XK_T) {
                        toggle_layout(display, &windows, &workspace_of, current_workspace, &mut tiled,
                            &mut saved_geometry, width, height, panel_top);
                        draw_panel(display, panel, gc, width, &config, current_workspace, tiled[current_workspace]);
                    } else {
                        let index = [XK_1, XK_2, XK_3, XK_4, XK_5, XK_6, XK_7, XK_8, XK_9]
                            .iter().position(|ks| e.keycode == keycode(display, *ks));
                        if let Some(target) = index {
                            switch_workspace(display, root, panel, gc, &ewmh, &config, &windows, &workspace_of,
                                &mut current_workspace, target, &tiled, width, height, panel_top, &mut focused);
                        }
                    }
                }
                CLIENT_MESSAGE => {
                    let message = event.client_message;
                    if message.message_type == ewmh.current_desktop && message.window == root {
                        let target = message.data.l[0].max(0) as usize;
                        if target < 9 {
                            switch_workspace(display, root, panel, gc, &ewmh, &config, &windows, &workspace_of,
                                &mut current_workspace, target, &tiled, width, height, panel_top, &mut focused);
                        }
                    } else if message.message_type == ewmh.active_window && windows.contains(&message.window) {
                        if let Some(target) = workspace_of.get(&message.window).copied() {
                            if target != current_workspace {
                                switch_workspace(display, root, panel, gc, &ewmh, &config, &windows, &workspace_of,
                                    &mut current_workspace, target, &tiled, width, height, panel_top, &mut focused);
                            }
                            if focused != 0 { XSetWindowBorder(display, focused, COLOR_UNFOCUSED); }
                            focused = message.window;
                            set_ewmh_active_window(display, root, &ewmh, focused);
                            XRaiseWindow(display, focused);
                            XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                            XSetInputFocus(display, focused, 1, 0);
                            XRaiseWindow(display, panel);
                        }
                    } else if message.message_type == ewmh.window_desktop && windows.contains(&message.window) {
                        let target = message.data.l[0].max(0) as usize;
                        if target < 9 {
                            workspace_of.insert(message.window, target);
                            set_ewmh_desktop(display, &ewmh, message.window, target);
                            if target == current_workspace { XMapWindow(display, message.window); }
                            else { XUnmapWindow(display, message.window); }
                            if focused == message.window && target != current_workspace {
                                focused = 0;
                                set_ewmh_active_window(display, root, &ewmh, 0);
                                XSetInputFocus(display, root, 1, 0);
                            }
                            if tiled[current_workspace] { arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top); }
                        }
                    } else if message.message_type == ewmh.close_window && windows.contains(&message.window) {
                        request_close(display, message.window);
                    }
                }
                BUTTON_PRESS => {
                    let e = event.button;
                    if e.window == panel {
                        if width >= 900 && (510..=714).contains(&e.x) {
                            let target = ((e.x - 510) / 24).clamp(0, 8) as usize;
                            switch_workspace(display, root, panel, gc, &ewmh, &config, &windows, &workspace_of,
                                &mut current_workspace, target, &tiled, width, height, panel_top, &mut focused);
                        } else {
                            match e.x {
                                90..=163 => launch_terminal(&config),
                                164..=219 => launch_files(&config),
                                220..=275 => launch_web(&config),
                                276..=326 => launch_calculator(),
                                330..=394 => launch_menu(&config),
                                395..=485 => open_settings(),
                                _ => {}
                            }
                        }
                        XRaiseWindow(display, panel);
                    } else {
                        if e.state & MOD1_MASK != 0 && (e.button == 1 || e.button == 3) {
                            if tiled[current_workspace] {
                                toggle_layout(display, &windows, &workspace_of, current_workspace, &mut tiled,
                                    &mut saved_geometry, width, height, panel_top);
                                draw_panel(display, panel, gc, width, &config, current_workspace, tiled[current_workspace]);
                            }
                            focused = e.window;
                            set_ewmh_active_window(display, root, &ewmh, focused);
                            XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                            XAllowEvents(display, ASYNC_POINTER, e.time);
                            drag_window(display, panel, focused, e.button, e.x_root, e.y_root);
                        } else {
                            if focused != 0 { XSetWindowBorder(display, focused, COLOR_UNFOCUSED); }
                            focused = e.window;
                            set_ewmh_active_window(display, root, &ewmh, focused);
                            XRaiseWindow(display, focused);
                            XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                            XSetInputFocus(display, focused, 1, 0);
                            XAllowEvents(display, REPLAY_POINTER, e.time);
                        }
                    }
                }
                EXPOSE => {
                    if event.expose.window == panel { draw_panel(display, panel, gc, width, &config, current_workspace, tiled[current_workspace]); }
                }
                DESTROY_NOTIFY => {
                    let win = event.destroy.window;
                    windows.remove(&win); minimized.retain(|window| *window != win);
                    workspace_of.remove(&win); saved_geometry.remove(&win);
                    if focused == win {
                        if let Some(next) = windows.iter().find(|&&w| workspace_of.get(&w).copied() == Some(current_workspace)) {
                            focused = *next; set_ewmh_active_window(display, root, &ewmh, focused); XSetInputFocus(display, focused, 1, 0);
                        } else { focused = 0; set_ewmh_active_window(display, root, &ewmh, focused); XSetInputFocus(display, root, 1, 0); }
                        if tiled[current_workspace] { arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top); }
                    }
                }
                _ => {}
            }
        }
    }
}
