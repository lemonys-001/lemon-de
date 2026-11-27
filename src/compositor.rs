//! A small, event-driven X11 compositor using Composite, Damage, Fixes, and Render.
//! It intentionally provides basic alpha compositing only: no blur, shadows, or animation.
use std::{collections::{HashMap, HashSet}, ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, CStr, CString}, ptr};

#[repr(C)] struct Display { _private: [u8; 0] }
type Window = c_ulong;
type Pixmap = c_ulong;
type Picture = c_ulong;
type Damage = c_ulong;
type Region = c_ulong;
type Visual = c_void;
type Format = *mut c_void;
#[repr(C)] #[derive(Clone, Copy)] struct XRectangle { x: i16, y: i16, width: u16, height: u16 }
#[repr(C)] struct XWindowAttributes {
    x: c_int, y: c_int, width: c_int, height: c_int, border_width: c_int, depth: c_int,
    visual: *mut Visual, root: Window, class: c_int, bit_gravity: c_int, win_gravity: c_int,
    backing_store: c_int, backing_planes: c_ulong, backing_pixel: c_ulong, save_under: c_int,
    colormap: c_ulong, map_installed: c_int, map_state: c_int, all_event_masks: c_long,
    your_event_mask: c_long, do_not_propagate_mask: c_long, override_redirect: c_int,
    screen: *mut c_void,
}
#[repr(C)] union XEvent { type_: c_int, pad: [c_long; 24] }
#[repr(C)] struct PollFd { fd: c_int, events: i16, revents: i16 }

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(name: *const c_char) -> *mut Display;
    fn XCloseDisplay(display: *mut Display) -> c_int;
    fn XDefaultRootWindow(display: *mut Display) -> Window;
    fn XDefaultVisual(display: *mut Display, screen: c_int) -> *mut Visual;
    fn XDefaultScreen(display: *mut Display) -> c_int;
    fn XDisplayWidth(display: *mut Display, screen: c_int) -> c_int;
    fn XDisplayHeight(display: *mut Display, screen: c_int) -> c_int;
    fn XSelectInput(display: *mut Display, window: Window, mask: c_long) -> c_int;
    fn XGetWindowAttributes(display: *mut Display, window: Window, attrs: *mut XWindowAttributes) -> c_int;
    fn XQueryTree(display: *mut Display, window: Window, root: *mut Window, parent: *mut Window,
        children: *mut *mut Window, count: *mut c_uint) -> c_int;
    fn XFree(data: *mut c_void) -> c_int;
    fn XFreePixmap(display: *mut Display, pixmap: Pixmap) -> c_int;
    fn XStoreName(display: *mut Display, window: Window, name: *const c_char) -> c_int;
    fn XFetchName(display: *mut Display, window: Window, name: *mut *mut c_char) -> c_int;
    fn XKillClient(display: *mut Display, resource: c_ulong) -> c_int;
    fn XSync(display: *mut Display, discard: c_int) -> c_int;
    fn XTranslateCoordinates(display: *mut Display, src: Window, dst: Window, src_x: c_int, src_y: c_int,
        dst_x: *mut c_int, dst_y: *mut c_int, child: *mut Window) -> c_int;
    fn XInternAtom(display: *mut Display, name: *const c_char, only_if_exists: c_int) -> c_ulong;
    fn XSetSelectionOwner(display: *mut Display, selection: c_ulong, owner: Window, time: c_ulong) -> c_int;
    fn XGetSelectionOwner(display: *mut Display, selection: c_ulong) -> Window;
    fn XConnectionNumber(display: *mut Display) -> c_int;
    fn XPending(display: *mut Display) -> c_int;
    fn XNextEvent(display: *mut Display, event: *mut XEvent) -> c_int;
    fn XFlush(display: *mut Display) -> c_int;
    fn XMapWindow(display: *mut Display, window: Window) -> c_int;
    fn poll(fds: *mut PollFd, count: c_ulong, timeout_ms: c_int) -> c_int;
}
#[link(name = "Xcomposite")]
extern "C" {
    fn XCompositeQueryExtension(display: *mut Display, event_base: *mut c_int, error_base: *mut c_int) -> c_int;
    fn XCompositeRedirectSubwindows(display: *mut Display, window: Window, update: c_int);
    fn XCompositeGetOverlayWindow(display: *mut Display, window: Window) -> Window;
    fn XCompositeReleaseOverlayWindow(display: *mut Display, window: Window);
    fn XCompositeNameWindowPixmap(display: *mut Display, window: Window) -> Pixmap;
}
#[link(name = "Xrender")]
extern "C" {
    fn XRenderQueryExtension(display: *mut Display, event_base: *mut c_int, error_base: *mut c_int) -> c_int;
    fn XRenderFindVisualFormat(display: *mut Display, visual: *mut Visual) -> Format;
    fn XRenderCreatePicture(display: *mut Display, drawable: c_ulong, format: Format,
        value_mask: c_ulong, attributes: *mut c_void) -> Picture;
    fn XRenderFreePicture(display: *mut Display, picture: Picture);
    fn XRenderComposite(display: *mut Display, op: c_int, src: Picture, mask: Picture,
        dst: Picture, src_x: c_int, src_y: c_int, mask_x: c_int, mask_y: c_int,
        dst_x: c_int, dst_y: c_int, width: c_uint, height: c_uint);
    fn XRenderFillRectangle(display: *mut Display, op: c_int, dst: Picture, color: *const XRenderColor,
        x: c_int, y: c_int, width: c_uint, height: c_uint);
}
#[repr(C)] struct XRenderColor { red: u16, green: u16, blue: u16, alpha: u16 }
#[link(name = "Xfixes")]
extern "C" {
    fn XFixesQueryExtension(display: *mut Display, event_base: *mut c_int, error_base: *mut c_int) -> c_int;
    fn XFixesCreateRegion(display: *mut Display, rectangles: *const XRectangle, count: c_int) -> Region;
    fn XFixesSetWindowShapeRegion(display: *mut Display, window: Window, shape_kind: c_int,
        x_offset: c_int, y_offset: c_int, region: Region);
    fn XFixesDestroyRegion(display: *mut Display, region: Region);
}
#[link(name = "Xdamage")]
extern "C" {
    fn XDamageQueryExtension(display: *mut Display, event_base: *mut c_int, error_base: *mut c_int) -> c_int;
    fn XDamageCreate(display: *mut Display, drawable: Window, level: c_int) -> Damage;
    fn XDamageDestroy(display: *mut Display, damage: Damage);
    fn XDamageSubtract(display: *mut Display, damage: Damage, repair: Region, parts: Region);
}
const SUBSTRUCTURE_NOTIFY: c_long = 1 << 19;
const STRUCTURE_NOTIFY: c_long = 1 << 17;
const IS_VIEWABLE: c_int = 2;
const INPUT_ONLY: c_int = 2;
const SHAPE_INPUT: c_int = 2;
const DAMAGE_REPORT_NON_EMPTY: c_int = 3;
const PICT_OP_SRC: c_int = 1;
const PICT_OP_OVER: c_int = 3;
const OWNER_TITLE: &str = "LDE Lite Compositor";

struct Layer { pixmap: Pixmap, picture: Picture, damage: Damage }
unsafe fn release_layer(display: *mut Display, layer: Layer) {
    XDamageDestroy(display, layer.damage);
    XRenderFreePicture(display, layer.picture);
    XFreePixmap(display, layer.pixmap);
}
unsafe fn get_or_create_layer(display: *mut Display, window: Window, layers: &mut HashMap<Window, Layer>) -> bool {
    if layers.contains_key(&window) { return true; }
    let mut attrs: XWindowAttributes = std::mem::zeroed();
    if XGetWindowAttributes(display, window, &mut attrs) == 0 || attrs.class == INPUT_ONLY
        || attrs.map_state != IS_VIEWABLE || attrs.width < 1 || attrs.height < 1 { return false; }
    let format = XRenderFindVisualFormat(display, attrs.visual);
    if format.is_null() { return false; }
    let pixmap = XCompositeNameWindowPixmap(display, window);
    if pixmap == 0 { return false; }
    let picture = XRenderCreatePicture(display, pixmap, format, 0, ptr::null_mut());
    if picture == 0 { XFreePixmap(display, pixmap); return false; }
    let damage = XDamageCreate(display, window, DAMAGE_REPORT_NON_EMPTY);
    if damage == 0 { XRenderFreePicture(display, picture); XFreePixmap(display, pixmap); return false; }
    XSelectInput(display, window, STRUCTURE_NOTIFY);
    layers.insert(window, Layer { pixmap, picture, damage });
    true
}
unsafe fn repaint(display: *mut Display, root: Window, overlay: Window, target: Picture,
                  width: c_int, height: c_int, layers: &mut HashMap<Window, Layer>) {
    let transparent = XRenderColor { red: 0, green: 0, blue: 0, alpha: 0 };
    XRenderFillRectangle(display, PICT_OP_SRC, target, &transparent, 0, 0, width as c_uint, height as c_uint);
    let mut tree_root = 0; let mut parent = 0; let mut children: *mut Window = ptr::null_mut(); let mut count = 0;
    let mut visible = HashSet::new();
    if XQueryTree(display, root, &mut tree_root, &mut parent, &mut children, &mut count) != 0 {
        for index in 0..count as usize {
            let window = *children.add(index);
            if window == overlay { continue; }
            let mut attrs: XWindowAttributes = std::mem::zeroed();
            if XGetWindowAttributes(display, window, &mut attrs) == 0 || attrs.class == INPUT_ONLY
                || attrs.map_state != IS_VIEWABLE || attrs.width < 1 || attrs.height < 1 { continue; }
            if !get_or_create_layer(display, window, layers) { continue; }
            visible.insert(window);
            let Some(layer) = layers.get(&window) else { continue };
            let (mut x, mut y) = (0, 0); let mut child = 0;
            XTranslateCoordinates(display, window, root, 0, 0, &mut x, &mut y, &mut child);
            XRenderComposite(display, PICT_OP_OVER, layer.picture, 0, target, 0, 0, 0, 0,
                x, y, attrs.width.max(0) as c_uint, attrs.height.max(0) as c_uint);
            XDamageSubtract(display, layer.damage, 0, 0);
        }
        if !children.is_null() { XFree(children.cast()); }
    }
    let stale: Vec<Window> = layers.keys().filter(|window| !visible.contains(window)).copied().collect();
    for window in stale { if let Some(layer) = layers.remove(&window) { release_layer(display, layer); } }
    XFlush(display);
}

/// Toggle this compositor only. An unrelated compositor selection is never killed.
pub fn toggle() -> Result<(), String> {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() { return Err("Cannot open X display".into()); }
        let screen_no = XDefaultScreen(display) as u32;
        let selection = XInternAtom(display, CString::new(format!("_NET_WM_CM_S{screen_no}")).unwrap().as_ptr(), 0);
        let owner = XGetSelectionOwner(display, selection);
        if owner != 0 {
            let mut name: *mut c_char = ptr::null_mut();
            let ours = XFetchName(display, owner, &mut name) != 0 && !name.is_null()
                && CStr::from_ptr(name).to_bytes() == OWNER_TITLE.as_bytes();
            if !name.is_null() { XFree(name.cast()); }
            if !ours { XCloseDisplay(display); return Err("Another X11 compositor already owns this screen".into()); }
            XKillClient(display, owner);
            XSync(display, 0);
            XCloseDisplay(display);
            eprintln!("Stopped LDE lite compositor");
            return Ok(());
        }
        XCloseDisplay(display);
    }
    run()
}

/// Start the small X11 compositor loop. This is opt-in via `lemon --compositor`.
pub fn run() -> Result<(), String> {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() { return Err("Cannot open X display".into()); }
        let root = XDefaultRootWindow(display);
        let screen = XDefaultScreen(display);
        let width = XDisplayWidth(display, screen).max(1);
        let height = XDisplayHeight(display, screen).max(1);
        let (mut event_base, mut error_base) = (0, 0);
        if XCompositeQueryExtension(display, &mut event_base, &mut error_base) == 0
            || XRenderQueryExtension(display, &mut event_base, &mut error_base) == 0
            || XFixesQueryExtension(display, &mut event_base, &mut error_base) == 0
            || XDamageQueryExtension(display, &mut event_base, &mut error_base) == 0 {
            XCloseDisplay(display);
            return Err("This X server needs Composite, Render, Fixes, and Damage extensions".into());
        }
        let selection = XInternAtom(display, CString::new(format!("_NET_WM_CM_S{}", screen as u32)).unwrap().as_ptr(), 0);
        if XGetSelectionOwner(display, selection) != 0 {
            XCloseDisplay(display);
            return Err("Another X11 compositor already owns this screen".into());
        }
        let owner = XCreateSimpleWindow(display, root, -1, -1, 1, 1, 0, 0, 0);
        XStoreName(display, owner, CString::new(OWNER_TITLE).unwrap().as_ptr());
        XSetSelectionOwner(display, selection, owner, 0);
        if XGetSelectionOwner(display, selection) != owner {
            XCloseDisplay(display);
            return Err("Another X11 compositor already owns this screen".into());
        }
        let overlay = XCompositeGetOverlayWindow(display, root);
        if overlay == 0 { XSetSelectionOwner(display, selection, 0, 0); XCloseDisplay(display); return Err("Cannot acquire the X11 compositor overlay".into()); }
        let empty_region = XFixesCreateRegion(display, ptr::null(), 0);
        XFixesSetWindowShapeRegion(display, overlay, SHAPE_INPUT, 0, 0, empty_region);
        XFixesDestroyRegion(display, empty_region);
        let mut overlay_attrs: XWindowAttributes = std::mem::zeroed();
        if XGetWindowAttributes(display, overlay, &mut overlay_attrs) == 0 {
            XCompositeReleaseOverlayWindow(display, root); XSetSelectionOwner(display, selection, 0, 0); XCloseDisplay(display);
            return Err("Cannot inspect the X11 compositor overlay visual".into());
        }
        let format = XRenderFindVisualFormat(display, overlay_attrs.visual);
        if format.is_null() {
            XCompositeReleaseOverlayWindow(display, root); XSetSelectionOwner(display, selection, 0, 0); XCloseDisplay(display);
            return Err("X server does not expose a Render visual for the compositor overlay".into());
        }
        let target = XRenderCreatePicture(display, overlay, format, 0, ptr::null_mut());
        if target == 0 {
            XCompositeReleaseOverlayWindow(display, root); XSetSelectionOwner(display, selection, 0, 0); XCloseDisplay(display);
            return Err("Cannot create the compositor's Render surface".into());
        }
        XSelectInput(display, root, SUBSTRUCTURE_NOTIFY | STRUCTURE_NOTIFY);
        XCompositeRedirectSubwindows(display, root, 1);
        XMapWindow(display, overlay);
        let mut damage_event_base = 0; let mut damage_error_base = 0;
        XDamageQueryExtension(display, &mut damage_event_base, &mut damage_error_base);
        let mut layers = HashMap::new();
        repaint(display, root, overlay, target, width, height, &mut layers);
        eprintln!("LDE lite compositor active ({width}x{height}); basic XRender alpha composition, no blur or shadows");
        let connection = XConnectionNumber(display);
        loop {
            if XPending(display) == 0 {
                let mut fd = PollFd { fd: connection, events: 1, revents: 0 };
                let ready = poll(&mut fd, 1, 1000);
                if ready < 0 { continue; }
                if ready == 0 { repaint(display, root, overlay, target, width, height, &mut layers); continue; }
            }
            let mut event: XEvent = std::mem::zeroed();
            XNextEvent(display, &mut event);
            let event_type = event.type_;
            let damage_notify = event_type == damage_event_base;
            if event_type == 22 {
                // Resized clients may receive a new redirected pixmap; rebuild cached pictures.
                for (_, layer) in layers.drain() { release_layer(display, layer); }
            }
            if damage_notify || matches!(event_type, 16 | 17 | 18 | 19 | 21 | 22 | 26 | 28) {
                repaint(display, root, overlay, target, width, height, &mut layers);
            }
        }
    }
}

#[link(name = "X11")]
extern "C" { fn XCreateSimpleWindow(display: *mut Display, parent: Window, x: c_int, y: c_int,
    width: u32, height: u32, border_width: u32, border: c_ulong, background: c_ulong) -> Window; }
