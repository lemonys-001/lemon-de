//! Small X11 window manager/shell. Core has no third-party Rust dependencies;
//! optional desktop services are separate user processes configured in ~/.config/lde/config.
mod config;
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
    fn XGetGeometry(display: *mut Display, drawable: Window, root: *mut Window,
        x: *mut c_int, y: *mut c_int, width: *mut c_uint, height: *mut c_uint,
        border_width: *mut c_uint, depth: *mut c_uint) -> c_int;
    fn XGrabPointer(display: *mut Display, grab_window: Window, owner_events: c_int,
        event_mask: c_uint, pointer_mode: c_int, keyboard_mode: c_int,
        confine_to: Window, cursor: c_ulong, time: c_ulong) -> c_int;
    fn XUngrabPointer(display: *mut Display, time: c_ulong) -> c_int;
}

const KEY_PRESS: c_int = 2;
const BUTTON_PRESS: c_int = 4;
const BUTTON_RELEASE: c_int = 5;
const MOTION_NOTIFY: c_int = 6;
const EXPOSE: c_int = 12;
const DESTROY_NOTIFY: c_int = 17;
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
const MOD4_MASK: c_uint = 1 << 6;
const SHIFT_MASK: c_uint = 1;
const XK_RETURN: c_ulong = 0xff0d;
const XK_D: c_ulong = 0x64;
const XK_I: c_ulong = 0x69;
const XK_Q: c_ulong = 0x71;
const XK_T: c_ulong = 0x74;
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
const COLOR_FOCUS: c_ulong = 0xffb84d;
const COLOR_UNFOCUSED: c_ulong = 0x505566;

#[derive(Clone, Copy)]
struct Geometry { x: c_int, y: c_int, width: c_uint, height: c_uint }

fn cstring(s: &str) -> CString { CString::new(s).unwrap_or_else(|_| CString::new(" ").unwrap()) }
fn shell_command(command: &str) -> bool {
    if command.trim().is_empty() { return false; }
    Command::new("sh").arg("-lc").arg(command).stdin(Stdio::null()).spawn().is_ok()
}
fn spawn_candidates(candidates: &[&str]) -> bool {
    for candidate in candidates {
        if Command::new(candidate).stdin(Stdio::null()).spawn().is_ok() { return true; }
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
fn launch_files(config: &Config) {
    if (!config.file_manager.is_empty() && shell_command(&config.file_manager))
        || spawn_candidates(&["thunar", "pcmanfm", "nemo", "dolphin"]) { return; }
    eprintln!("LDE: no file manager found; install one or set file_manager in the LDE config.");
}
fn launch_web(config: &Config) {
    if (!config.browser.is_empty() && shell_command(&config.browser))
        || spawn_candidates(&["firefox", "chromium", "chromium-browser", "google-chrome"]) { return; }
    eprintln!("LDE: no browser found; install one or set browser in the LDE config.");
}
fn launch_menu(config: &Config) {
    if (!config.launcher.is_empty() && shell_command(&config.launcher))
        || Command::new("rofi").args(["-show", "drun"]).stdin(Stdio::null()).spawn().is_ok()
        || spawn_candidates(&["dmenu_run", "bemenu-run"]) { return; }
    for term in ["foot", "kitty", "alacritty", "xterm", "urxvt"] {
        if Command::new(term).args(["-e", "sh", "-lc", "printf 'LDE launcher\\n\\nInstall rofi or dmenu for an application menu.\\nHotkeys: Super+Enter terminal, Super+1..9 workspaces.\\n'; exec \"${SHELL:-/bin/sh}\""]).spawn().is_ok() { return; }
    }
    eprintln!("LDE: install rofi or dmenu_run for the application launcher.");
}
fn shell_quote(value: &str) -> String { format!("'{}'", value.replace('\'', "'\\''")) }
fn open_settings() {
    let path = match Config::ensure_file() {
        Ok(path) => path,
        Err(error) => { eprintln!("LDE: {error}"); return; }
    };
    let path_arg = path.to_string_lossy().to_string();
    for editor in ["mousepad", "featherpad", "xed", "geany", "leafpad", "gedit"] {
        if Command::new(editor).arg(&path).stdin(Stdio::null()).spawn().is_ok() { return; }
    }
    let command = format!("${{EDITOR:-vi}} {}", shell_quote(&path_arg));
    let mut terminals: Vec<String> = std::env::var("TERMINAL").ok().into_iter().collect();
    terminals.extend(["foot", "kitty", "alacritty", "xterm", "urxvt"].into_iter().map(str::to_owned));
    for term in terminals {
        if Command::new(&term).arg("-e").arg("sh").arg("-lc").arg(&command)
            .stdin(Stdio::null()).spawn().is_ok() { return; }
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

unsafe fn draw_panel(display: *mut Display, panel: Window, gc: GC, width: c_int,
                     config: &Config, current: usize, tiled: bool) {
    XSetForeground(display, gc, config.panel_color as c_ulong);
    XFillRectangle(display, panel, gc, 0, 0, width.max(1) as c_uint, PANEL_H as c_uint);
    XSetForeground(display, gc, config.accent_color as c_ulong);
    XFillRectangle(display, panel, gc, 0, 0, 4, PANEL_H as c_uint);
    XSetForeground(display, gc, COLOR_TEXT);
    let title = cstring("LEMON");
    XDrawString(display, panel, gc, 16, 22, title.as_ptr(), 5);
    XSetForeground(display, gc, COLOR_DIM);
    let buttons = cstring("  Terminal    Files    Web    Apps");
    XDrawString(display, panel, gc, 98, 22, buttons.as_ptr(), 34);

    if width >= 700 {
        for i in 0..9 {
            let x = 425 + i as c_int * 24;
            if i == current {
                XSetForeground(display, gc, config.accent_color as c_ulong);
                XFillRectangle(display, panel, gc, x - 5, 4, 20, 26);
                XSetForeground(display, gc, config.panel_color as c_ulong);
            } else { XSetForeground(display, gc, COLOR_DIM); }
            let label = cstring(&(i + 1).to_string());
            XDrawString(display, panel, gc, x, 22, label.as_ptr(), 1);
        }
    }
    let mode = cstring(if tiled { "TILE" } else { "FLOAT" });
    XSetForeground(display, gc, config.accent_color as c_ulong);
    XDrawString(display, panel, gc, (width - 150).max(8), 22, mode.as_ptr(), 4 + if tiled { 0 } else { 1 });
    let clock = chrono_like_clock();
    let clock_c = cstring(&clock);
    XSetForeground(display, gc, COLOR_TEXT);
    XDrawString(display, panel, gc, (width - 76).max(8), 22, clock_c.as_ptr(), clock.len() as c_int);
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
                           config: &Config, windows: &HashSet<Window>,
                           workspace_of: &HashMap<Window, usize>, current: &mut usize,
                           target: usize, tiled: &[bool; 9], width: c_int, height: c_int,
                           panel_top: bool, focused: &mut Window) {
    if target >= 9 || target == *current { return; }
    let old = *current;
    for window in windows {
        if workspace_of.get(window).copied() == Some(old) { XUnmapWindow(display, *window); }
    }
    *current = target;
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
    XRaiseWindow(display, panel);
    draw_panel(display, panel, gc, width, config, target, tiled[target]);
}
unsafe fn drag_window(display: *mut Display, root: Window, panel: Window, window: Window,
                      button: c_uint, start_x: c_int, start_y: c_int) {
    let Some(initial) = geometry(display, window) else { return; };
    let mask = (POINTER_MOTION_MASK | BUTTON_RELEASE_MASK) as c_uint;
    if XGrabPointer(display, root, 0, mask, 1, 1, 0, 0, 0) != 0 { return; }
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
        println!("LDE — lightweight X11 desktop shell and tools\n\nUsage: lemon [--menu|--no-menu|--settings|--install|--launcher|--files [PATH]]\n       lemon --help | --version | --config-path | --init-config\n\nWith no arguments in a terminal without DISPLAY, Lemon opens its interactive menu. If DISPLAY is set (including Termux/VNC), it starts the X11 desktop; use --menu to force the terminal menu.\nShortcuts: Super+Enter terminal, Super+D launcher, Super+I settings, Super+T layout, Super+Q close, Super+Shift+Q quit, Super+1..9 workspaces."); return;
    }
    if args.iter().any(|a| a == "--config-path") { println!("{}", Config::path().display()); return; }
    if args.iter().any(|a| a == "--init-config") {
        match Config::ensure_file() { Ok(path) => println!("LDE config ready: {}", path.display()), Err(e) => { eprintln!("LDE: {e}"); std::process::exit(1); } }
        return;
    }
    if args.iter().any(|a| a == "--settings") {
        if terminal_ui::stdin_is_terminal() || std::env::var_os("DISPLAY").is_none() {
            terminal_ui::settings();
        } else {
            open_settings();
        }
        return;
    }
    if args.iter().any(|a| a == "--install") { terminal_ui::install_components(); return; }
    if args.iter().any(|a| a == "--launcher") { terminal_ui::launcher(); return; }
    if args.iter().any(|a| a == "--files") { terminal_ui::file_browser(); return; }
    let display_is_set = std::env::var_os("DISPLAY").is_some();
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
        let width = XDisplayWidth(display, screen);
        let height = XDisplayHeight(display, screen);
        XSelectInput(display, root, SUBSTRUCTURE_REDIRECT_MASK | SUBSTRUCTURE_NOTIFY_MASK | KEY_PRESS_MASK);
        if config.wallpaper.is_empty() { XSetWindowBackground(display, root, config.background_color as c_ulong); }
        XClearWindow(display, root);
        for key in [XK_RETURN, XK_D, XK_I, XK_Q, XK_T, XK_1, XK_2, XK_3, XK_4, XK_5, XK_6, XK_7, XK_8, XK_9] {
            grab(display, root, key, MOD4_MASK);
        }
        grab(display, root, XK_Q, MOD4_MASK | SHIFT_MASK);

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
        loop {
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
                    let slot = next_slot % 7;
                    next_slot += 1;
                    let margin = 24 + (slot as c_int * 28);
                    let area_y = if panel_top { PANEL_H } else { 0 };
                    let client_w = ((width - margin * 2).min(width * 3 / 4)).max(320) as c_uint;
                    let client_h = ((height - PANEL_H - margin * 2).min(height * 3 / 4)).max(240) as c_uint;
                    let initial = Geometry { x: margin, y: area_y + margin, width: client_w, height: client_h };
                    XSelectInput(display, e.window, BUTTON_PRESS_MASK);
                    XSetWindowBorderWidth(display, e.window, 2);
                    XSetWindowBorder(display, e.window, config.accent_color as c_ulong);
                    XMoveResizeWindow(display, e.window, initial.x, initial.y, initial.width, initial.height);
                    XMapWindow(display, e.window);
                    windows.insert(e.window);
                    workspace_of.insert(e.window, current_workspace);
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
                    if e.state & MOD4_MASK == 0 { continue; }
                    if e.keycode == keycode(display, XK_RETURN) { launch_terminal(&config); }
                    else if e.keycode == keycode(display, XK_D) { launch_menu(&config); }
                    else if e.keycode == keycode(display, XK_I) { open_settings(); }
                    else if e.keycode == keycode(display, XK_Q) && e.state & SHIFT_MASK != 0 {
                        XCloseDisplay(display); return;
                    } else if e.keycode == keycode(display, XK_Q) && focused != 0 {
                        XKillClient(display, focused);
                    } else if e.keycode == keycode(display, XK_T) {
                        toggle_layout(display, &windows, &workspace_of, current_workspace, &mut tiled,
                            &mut saved_geometry, width, height, panel_top);
                        draw_panel(display, panel, gc, width, &config, current_workspace, tiled[current_workspace]);
                    } else {
                        let index = [XK_1, XK_2, XK_3, XK_4, XK_5, XK_6, XK_7, XK_8, XK_9]
                            .iter().position(|ks| e.keycode == keycode(display, *ks));
                        if let Some(target) = index {
                            switch_workspace(display, root, panel, gc, &config, &windows, &workspace_of,
                                &mut current_workspace, target, &tiled, width, height, panel_top, &mut focused);
                        }
                    }
                }
                BUTTON_PRESS => {
                    let e = event.button;
                    if e.window == panel {
                        if width >= 700 && (425..=640).contains(&e.x) {
                            let target = ((e.x - 425) / 24).clamp(0, 8) as usize;
                            switch_workspace(display, root, panel, gc, &config, &windows, &workspace_of,
                                &mut current_workspace, target, &tiled, width, height, panel_top, &mut focused);
                        } else {
                            match e.x {
                                70..=174 => launch_terminal(&config),
                                175..=250 => launch_files(&config),
                                251..=310 => launch_web(&config),
                                311..=420 => launch_menu(&config),
                                _ => {}
                            }
                        }
                        XRaiseWindow(display, panel);
                    } else {
                        if e.state & MOD1_MASK != 0 && (e.button == 1 || e.button == 3) {
                            if tiled[current_workspace] {
                                toggle_layout(display, &windows, &workspace_of, current_workspace, &mut tiled,
                                    &mut saved_geometry, width, height, panel_top);
                            }
                            focused = e.window;
                            XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                            drag_window(display, root, panel, focused, e.button, e.x_root, e.y_root);
                        } else {
                            if focused != 0 { XSetWindowBorder(display, focused, COLOR_UNFOCUSED); }
                            focused = e.window;
                            XRaiseWindow(display, focused);
                            XSetWindowBorder(display, focused, config.accent_color as c_ulong);
                            XSetInputFocus(display, focused, 1, 0);
                        }
                    }
                }
                EXPOSE => {
                    if event.expose.window == panel { draw_panel(display, panel, gc, width, &config, current_workspace, tiled[current_workspace]); }
                }
                DESTROY_NOTIFY => {
                    let win = event.destroy.window;
                    windows.remove(&win); workspace_of.remove(&win); saved_geometry.remove(&win);
                    if focused == win {
                        if let Some(next) = windows.iter().find(|&&w| workspace_of.get(&w).copied() == Some(current_workspace)) {
                            focused = *next; XSetInputFocus(display, focused, 1, 0);
                        } else { focused = 0; XSetInputFocus(display, root, 1, 0); }
                        if tiled[current_workspace] { arrange_tiled(display, &windows, &workspace_of, current_workspace, width, height, panel_top); }
                    }
                }
                _ => {}
            }
        }
    }
}
