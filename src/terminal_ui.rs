use crate::config::Config;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Clone, Copy)]
struct Package {
    name: &'static str,
    description: &'static str,
    apt: &'static str,
    dnf: &'static str,
    pacman: &'static str,
    zypper: &'static str,
    apk: &'static str,
    pkg: &'static str,
    setting: &'static str,
    command: &'static str,
}

const PACKAGES: &[Package] = &[
    Package { name: "Terminal", description: "Lightweight X terminal emulator", apt: "xterm", dnf: "xterm", pacman: "xterm", zypper: "xterm", apk: "xterm", pkg: "xterm", setting: "terminal", command: "xterm" },
    Package { name: "File manager", description: "PCManFM graphical file manager", apt: "pcmanfm", dnf: "pcmanfm", pacman: "pcmanfm", zypper: "pcmanfm", apk: "pcmanfm", pkg: "pcmanfm", setting: "file_manager", command: "pcmanfm" },
    Package { name: "Browser", description: "Firefox web browser", apt: "firefox", dnf: "firefox", pacman: "firefox", zypper: "MozillaFirefox", apk: "firefox", pkg: "firefox", setting: "browser", command: "firefox" },
    Package { name: "Graphical launcher", description: "Rofi application launcher", apt: "rofi", dnf: "rofi", pacman: "rofi", zypper: "rofi", apk: "rofi", pkg: "rofi", setting: "launcher", command: "rofi -show drun" },
    Package { name: "Notifications", description: "Dunst notification daemon (optional autostart)", apt: "dunst", dnf: "dunst", pacman: "dunst", zypper: "dunst", apk: "dunst", pkg: "dunst", setting: "", command: "dunst" },
    Package { name: "Wallpaper helper", description: "Set wallpaper from an image", apt: "feh", dnf: "feh", pacman: "feh", zypper: "feh", apk: "feh", pkg: "feh", setting: "", command: "" },
    Package { name: "Power manager", description: "XFCE power and battery controls", apt: "xfce4-power-manager", dnf: "xfce4-power-manager", pacman: "xfce4-power-manager", zypper: "xfce4-power-manager", apk: "xfce4-power-manager", pkg: "xfce4-power-manager", setting: "", command: "xfce4-power-manager" },
    Package { name: "Screenshooter", description: "XFCE screenshot utility", apt: "xfce4-screenshooter", dnf: "xfce4-screenshooter", pacman: "xfce4-screenshooter", zypper: "xfce4-screenshooter", apk: "xfce4-screenshooter", pkg: "xfce4-screenshooter", setting: "", command: "" },
    Package { name: "Clipboard manager", description: "XFCE clipboard history helper", apt: "xfce4-clipman", dnf: "xfce4-clipman", pacman: "xfce4-clipman-plugin", zypper: "xfce4-clipman", apk: "xfce4-clipman", pkg: "xfce4-clipman", setting: "", command: "xfce4-clipman" },
    Package { name: "Archive manager", description: "Xarchiver lightweight archive GUI", apt: "xarchiver", dnf: "xarchiver", pacman: "xarchiver", zypper: "xarchiver", apk: "xarchiver", pkg: "xarchiver", setting: "", command: "" },
    Package { name: "Appearance", description: "GTK theme and icon chooser", apt: "lxappearance", dnf: "lxappearance", pacman: "lxappearance", zypper: "lxappearance", apk: "lxappearance", pkg: "lxappearance", setting: "", command: "" },
    Package { name: "Volume control", description: "PulseAudio/PipeWire mixer UI", apt: "pavucontrol", dnf: "pavucontrol", pacman: "pavucontrol", zypper: "pavucontrol", apk: "pavucontrol", pkg: "pavucontrol", setting: "", command: "" },
    Package { name: "Display settings", description: "ARandR monitor layout tool", apt: "arandr", dnf: "arandr", pacman: "arandr", zypper: "arandr", apk: "arandr", pkg: "arandr", setting: "", command: "" },
    Package { name: "Compositor", description: "Optional Picom effects and shadows", apt: "picom", dnf: "picom", pacman: "picom", zypper: "picom", apk: "picom", pkg: "picom", setting: "", command: "picom" },
    Package { name: "Text editor", description: "Mousepad lightweight editor", apt: "mousepad", dnf: "mousepad", pacman: "mousepad", zypper: "mousepad", apk: "mousepad", pkg: "mousepad", setting: "", command: "" },
    Package { name: "Image viewer", description: "Ristretto lightweight image viewer", apt: "ristretto", dnf: "ristretto", pacman: "ristretto", zypper: "ristretto", apk: "ristretto", pkg: "ristretto", setting: "", command: "" },
    Package { name: "Task manager", description: "XFCE process monitor GUI", apt: "xfce4-taskmanager", dnf: "xfce4-taskmanager", pacman: "xfce4-taskmanager", zypper: "xfce4-taskmanager", apk: "xfce4-taskmanager", pkg: "xfce4-taskmanager", setting: "", command: "" },
    Package { name: "Screensaver/locker", description: "XFCE screensaver and lock helper", apt: "xfce4-screensaver", dnf: "xfce4-screensaver", pacman: "xfce4-screensaver", zypper: "xfce4-screensaver", apk: "xfce4-screensaver", pkg: "xfce4-screensaver", setting: "", command: "xfce4-screensaver" },
    Package { name: "Network tray", description: "NetworkManager GUI applet", apt: "network-manager-gnome", dnf: "NetworkManager-applet", pacman: "network-manager-applet", zypper: "NetworkManager-applet", apk: "network-manager-applet", pkg: "network-manager-applet", setting: "", command: "nm-applet" },
];

fn spawn_background(command: &mut Command) -> std::io::Result<std::process::Child> {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    command.spawn()
}

fn read_choice(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => return "0".to_owned(),
        Ok(_) => input.trim().to_owned(),
    }
}

fn save(config: &Config) {
    match config.save() {
        Ok(path) => println!("Saved {}", path.display()),
        Err(error) => eprintln!("Could not save settings: {error}"),
    }
}

fn set_setting(config: &mut Config, key: &str, value: String) {
    match key {
        "terminal" => config.terminal = value,
        "launcher" => config.launcher = value,
        "file_manager" => config.file_manager = value,
        "browser" => config.browser = value,
        "wallpaper" => config.wallpaper = value,
        "panel_position" if value == "top" || value == "bottom" => config.panel_position = value,
        "layout" if value == "floating" || value == "tiled" => config.layout = value,
        _ => return,
    }
    save(config);
}

pub fn settings() {
    let mut config = Config::load();
    loop {
        println!("\nLemon Settings  ·  {}\n", Config::path().display());
        println!(" 1. Terminal command       [{}]", config.terminal);
        println!(" 2. Application launcher   [{}]", config.launcher);
        println!(" 3. File manager           [{}]", config.file_manager);
        println!(" 4. Web browser            [{}]", config.browser);
        println!(" 5. Wallpaper image path   [{}]", config.wallpaper);
        println!(" 6. Panel position         [{}]", config.panel_position);
        println!(" 7. Default layout         [{}]", config.layout);
        println!(" 8. Panel RGB color        [0x{:06x}]", config.panel_color);
        println!(" 9. Accent RGB color       [0x{:06x}]", config.accent_color);
        println!("10. Background RGB color   [0x{:06x}]", config.background_color);
        println!("11. Add an autostart command (currently {})", config.autostart.len());
        println!("12. Clear autostart commands");
        println!(" 0. Back");
        match read_choice("Select a setting: ").as_str() {
            "1" => { let v = read_choice("Terminal command (blank = auto-detect): "); set_setting(&mut config, "terminal", v); }
            "2" => { let v = read_choice("Launcher command (blank = built-in auto-detection): "); set_setting(&mut config, "launcher", v); }
            "3" => { let v = read_choice("File manager command (blank = auto-detect): "); set_setting(&mut config, "file_manager", v); }
            "4" => { let v = read_choice("Browser command (blank = auto-detect): "); set_setting(&mut config, "browser", v); }
            "5" => { let v = read_choice("Wallpaper image path (blank = none): "); set_setting(&mut config, "wallpaper", v); }
            "6" => { let v = read_choice("Panel position [top/bottom]: "); set_setting(&mut config, "panel_position", v); }
            "7" => { let v = read_choice("Default layout [floating/tiled]: "); set_setting(&mut config, "layout", v); }
            "8" => edit_color(&mut config, "panel"),
            "9" => edit_color(&mut config, "accent"),
            "10" => edit_color(&mut config, "background"),
            "11" => {
                let v = read_choice("Autostart command (blank cancels): ");
                if !v.trim().is_empty() { config.autostart.push(v); save(&config); }
            }
            "12" => {
                if read_choice("Clear all autostart commands? [y/N] ").eq_ignore_ascii_case("y") {
                    config.autostart.clear(); save(&config);
                }
            }
            "0" => return,
            _ => println!("Choose one of the listed numbers."),
        }
    }
}

fn edit_color(config: &mut Config, which: &str) {
    let value = read_choice("RGB color (six hex digits, e.g. #ffb84d): ");
    let digits = value.trim().trim_start_matches('#').trim_start_matches("0x");
    match u64::from_str_radix(digits, 16).ok().filter(|color| *color <= 0xffffff) {
        Some(color) => {
            match which { "panel" => config.panel_color = color, "accent" => config.accent_color = color, _ => config.background_color = color }
            save(config);
        }
        None => println!("Invalid RGB color; expected a value from 000000 to ffffff."),
    }
}

fn package_manager() -> Option<(&'static str, &'static str)> {
    let candidates = [("apt-get", "Debian / Ubuntu"), ("dnf", "Fedora"), ("pacman", "Arch Linux"), ("zypper", "openSUSE"), ("apk", "Alpine Linux"), ("pkg", "Termux")];
    candidates.into_iter().find(|(program, _)| executable_on_path(program))
}

fn executable_on_path(program: &str) -> bool {
    if program.contains('/') { return Path::new(program).is_file(); }
    env::var_os("PATH").unwrap_or_default().to_string_lossy().split(':')
        .any(|dir| Path::new(dir).join(program).is_file())
}

fn package_name(package: &Package, manager: &str) -> &'static str {
    match manager {
        "apt-get" if package.name == "Browser" && is_debian() => "firefox-esr",
        "apt-get" => package.apt,
        "dnf" => package.dnf,
        "pacman" => package.pacman,
        "zypper" => package.zypper,
        "apk" => package.apk,
        _ => package.pkg,
    }
}

fn is_debian() -> bool {
    fs::read_to_string("/etc/os-release").unwrap_or_default().lines()
        .any(|line| line.trim() == "ID=debian")
}

pub fn install_components() {
    let Some((manager, distro)) = package_manager() else {
        println!("No supported package manager found (apt, dnf, pacman, zypper, apk, pkg).\nYou can still use Lemon's built-in launcher, file browser, and settings.");
        return;
    };
    println!("\nInstall optional desktop components · {distro}\n");
    for (index, package) in PACKAGES.iter().enumerate() {
        println!(" {}. {:<20} {}", index + 1, package.name, package.description);
    }
    println!("\nEnter numbers separated by spaces, or 0 to cancel.");
    let selected = read_choice("Packages: ");
    let indexes: BTreeSet<usize> = selected.split_whitespace().filter_map(|token| token.parse::<usize>().ok())
        .filter(|number| (1..=PACKAGES.len()).contains(number)).map(|number| number - 1).collect();
    if indexes.is_empty() { println!("Nothing selected."); return; }
    let chosen: Vec<&Package> = indexes.iter().map(|index| &PACKAGES[*index]).collect();
    println!("\nPackage manager will install: {}", chosen.iter().map(|p| package_name(p, manager)).collect::<Vec<_>>().join(" "));
    if !read_choice("Continue? [y/N] ").eq_ignore_ascii_case("y") { println!("Cancelled."); return; }

    let mut command = manager.to_owned();
    let mut args: Vec<String> = match manager {
        "apt-get" => vec!["install".into(), "-y".into()],
        "dnf" => vec!["install".into(), "-y".into()],
        "pacman" => vec!["-S".into(), "--needed".into(), "--noconfirm".into()],
        "zypper" => vec!["--non-interactive".into(), "install".into()],
        "apk" => vec!["add".into()],
        _ => vec!["install".into()],
    };
    args.extend(chosen.iter().map(|package| package_name(package, manager).to_owned()));
    if manager != "pkg" && !is_root() && executable_on_path("sudo") {
        args.insert(0, command);
        command = "sudo".into();
    }
    let status = Command::new(&command).args(&args).stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).status();
    match status {
        Ok(status) if status.success() => {
            let mut config = Config::load();
            let defaults: Vec<&Package> = chosen.iter().copied().filter(|p| !p.setting.is_empty()).collect();
            if !defaults.is_empty() {
                println!("\nSet newly installed components as Lemon defaults?");
                for package in defaults {
                    if read_choice(&format!("Use {} as the default {}? [Y/n] ", package.command, package.name)).to_lowercase() != "n" {
                        set_setting(&mut config, package.setting, package.command.to_owned());
                    }
                }
            }
            let mut add_autostart = false;
            for package in chosen.iter().filter(|package| package.setting.is_empty() && !package.command.is_empty()) {
                if !config.autostart.iter().any(|item| item == package.command)
                    && read_choice(&format!("Start {} automatically with Lemon? [y/N] ", package.command)).eq_ignore_ascii_case("y") {
                    config.autostart.push(package.command.to_owned());
                    add_autostart = true;
                }
            }
            if add_autostart { save(&config); }
            println!("Install complete.");
        }
        Ok(status) => eprintln!("Package manager exited with {status}."),
        Err(error) => eprintln!("Could not start {command}: {error}"),
    }
}

fn is_root() -> bool {
    Command::new("id").arg("-u").output().ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .is_some_and(|uid| uid.trim() == "0")
}

pub fn run_menu() {
    loop {
        println!("\n🍋 Lemon Desktop Environment\n");
        println!(" 1. Start the Lemon desktop (X11)");
        println!(" 2. Install additional desktop packages");
        println!(" 3. Lemon Settings");
        println!(" 4. Built-in application launcher");
        println!(" 5. Built-in file browser");
        println!(" 6. Built-in calculator");
        println!(" 7. Show configuration path");
        println!(" 0. Exit");
        match read_choice("Choose an option: ").as_str() {
            "1" => { crate::start_desktop(); return; }
            "2" => install_components(),
            "3" => settings(),
            "4" => launcher(),
            "5" => file_browser(),
            "6" => calculator(),
            "7" => println!("{}", Config::path().display()),
            "0" => return,
            _ => println!("Choose one of the listed numbers."),
        }
    }
}

pub fn launcher() {
    let mut apps = BTreeSet::new();
    for directory in env::var_os("PATH").unwrap_or_default().to_string_lossy().split(':') {
        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() && is_executable(&metadata) {
                        if let Some(name) = entry.file_name().to_str() { apps.insert(name.to_owned()); }
                    }
                }
            }
        }
    }
    let apps: Vec<String> = apps.into_iter().collect();
    if apps.is_empty() { println!("No executable applications found on PATH."); return; }
    println!("\nLemon Launcher · {} commands on PATH", apps.len());
    let query = read_choice("Filter (blank lists all): ").to_lowercase();
    let matches: Vec<&String> = apps.iter().filter(|app| app.to_lowercase().contains(&query)).take(40).collect();
    if matches.is_empty() { println!("No matching commands."); return; }
    for (index, app) in matches.iter().enumerate() { println!(" {:>2}. {app}", index + 1); }
    if let Ok(index) = read_choice("Launch number (0 to cancel): ").parse::<usize>() {
        if let Some(app) = index.checked_sub(1).and_then(|i| matches.get(i)) {
            let mut child = Command::new(app.as_str());
            child.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
            match spawn_background(&mut child) {
                Ok(_) => (), Err(error) => eprintln!("Could not launch {app}: {error}"),
            }
        }
    }
}

#[cfg(unix)]
fn is_executable(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}
#[cfg(not(unix))]
fn is_executable(_: &fs::Metadata) -> bool { false }

pub fn file_browser() {
    let mut directory = env::args_os().nth(2).map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    if directory.is_file() { open_file(&directory); return; }
    loop {
        println!("\nLemon Files · {}\n", directory.display());
        let mut entries: Vec<_> = match fs::read_dir(&directory) {
            Ok(entries) => entries.flatten().collect(),
            Err(error) => { eprintln!("Cannot read directory: {error}"); return; }
        };
        entries.sort_by_key(|entry| (!entry.path().is_dir(), entry.file_name()));
        println!(" 0. .. (parent directory)");
        for (index, entry) in entries.iter().enumerate().take(200) {
            let marker = if entry.path().is_dir() { "/" } else { "" };
            println!(" {}. {}{}", index + 1, entry.file_name().to_string_lossy(), marker);
        }
        println!("\nEnter a number to open / enter, 'q' to return.");
        let choice = read_choice("Selection: ");
        if choice.eq_ignore_ascii_case("q") { return; }
        let Ok(number) = choice.parse::<usize>() else { continue };
        if number == 0 { if let Some(parent) = directory.parent() { directory = parent.to_path_buf(); } continue; }
        let Some(entry) = entries.get(number - 1) else { println!("Out of range."); continue };
        let path = entry.path();
        if path.is_dir() { directory = path; } else { open_file(&path); }
    }
}

fn open_file(path: &Path) {
    let config = Config::load();
    let opener = if !config.file_manager.trim().is_empty() { Some(config.file_manager) }
        else if executable_on_path("xdg-open") { Some("xdg-open".into()) } else { None };
    if let Some(opener) = opener {
        let mut command = Command::new("sh");
        command.arg("-lc").arg(format!("{opener} \"$1\""))
            .arg("lemon-files").arg(path).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        if let Err(error) = spawn_background(&mut command) { eprintln!("Could not open {}: {error}", path.display()); }
    } else {
        println!("No graphical opener is available; path: {}", path.display());
    }
}

pub fn stdin_is_terminal() -> bool { io::stdin().is_terminal() }

/// Minimal terminal calculator; expression parsing is shared with the native X11 app.
pub fn calculator() {
    loop {
        let expression = read_choice("Lemon Calculator · enter expression or q to quit: ");
        if expression.eq_ignore_ascii_case("q") { return; }
        match crate::native_apps::evaluate(&expression) {
            Ok(value) => println!("= {value}"),
            Err(error) => println!("Invalid expression: {error}"),
        }
    }
}
