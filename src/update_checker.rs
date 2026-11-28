//! Low-overhead, once-a-day release checks. No network work runs on the UI thread.
use std::{
    env, fs::{self, OpenOptions}, io::Write, path::PathBuf,
    process::{Command, Stdio}, time::{Duration, SystemTime, UNIX_EPOCH},
};

const RELEASE_URL: &str = "https://api.github.com/repos/lemonys-001/lemon-de/releases/latest";
const CHECK_INTERVAL: u64 = 24 * 60 * 60;
const LOCK_MAX_AGE: u64 = 10 * 60;

#[derive(Default)]
struct State { checked_at: u64, latest: String, failed: bool }

fn cache_path() -> PathBuf {
    if let Some(path) = env::var_os("XDG_CACHE_HOME") { return PathBuf::from(path).join("lde/update-check"); }
    env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."))
        .join(".cache/lde/update-check")
}
fn now() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }
fn read_state() -> State {
    let Ok(text) = fs::read_to_string(cache_path()) else { return State::default() };
    let mut state = State::default();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "checked_at" => state.checked_at = value.parse().unwrap_or(0),
                "latest" => state.latest = value.trim().to_owned(),
                "failed" => state.failed = value == "true",
                _ => {}
            }
        }
    }
    state
}
fn write_state(state: &State) {
    let path = cache_path();
    let Some(parent) = path.parent() else { return };
    if fs::create_dir_all(parent).is_err() { return; }
    let temp = path.with_extension("tmp");
    if let Ok(mut file) = OpenOptions::new().create(true).truncate(true).write(true).open(&temp) {
        if write!(file, "checked_at={}\nlatest={}\nfailed={}\n", state.checked_at, state.latest, state.failed).is_ok() {
            let _ = file.sync_all();
            let _ = fs::rename(temp, path);
        }
    }
}
fn lock_path() -> PathBuf { cache_path().with_extension("lock") }
fn acquire_lock() -> bool {
    let path = lock_path();
    if let Some(parent) = path.parent() { if fs::create_dir_all(parent).is_err() { return false; } }
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let stale = fs::metadata(&path).ok().and_then(|m| m.modified().ok())
                .and_then(|t| t.elapsed().ok()).map(|age| age > Duration::from_secs(LOCK_MAX_AGE)).unwrap_or(false);
            stale && fs::remove_file(&path).is_ok()
                && OpenOptions::new().write(true).create_new(true).open(path).is_ok()
        }
        Err(_) => false,
    }
}
fn release_lock() { let _ = fs::remove_file(lock_path()); }

/// Schedule at most one detached checker when automatic checks are enabled and due.
pub fn schedule_if_due(enabled: bool) {
    if !enabled { return; }
    let state = read_state();
    if state.checked_at != 0 && now().saturating_sub(state.checked_at) < CHECK_INTERVAL { return; }
    if !acquire_lock() { return; }
    let Ok(executable) = env::current_exe() else { release_lock(); return };
    let mut command = Command::new(executable);
    command.arg("--update-check-worker").stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    #[cfg(unix)] {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    if command.spawn().is_err() { release_lock(); }
}

/// Check GitHub's latest stable release and update the local cache. Runs in the worker process.
pub fn run_worker() {
    let mut state = read_state();
    state.checked_at = now();
    state.failed = true;
    let result = Command::new("curl")
        .args(["--fail", "--silent", "--show-error", "--location", "--connect-timeout", "2",
            "--max-time", "5", "--max-filesize", "131072", "--user-agent", "LemonOS-update-checker/1",
            RELEASE_URL])
        .stdin(Stdio::null()).stderr(Stdio::null()).output();
    if let Ok(output) = result {
        if output.status.success() {
            if let Some(tag) = release_tag(&String::from_utf8_lossy(&output.stdout)) {
                state.latest = tag;
                state.failed = false;
            }
        }
    }
    let update_available = !state.failed && is_newer(&state.latest, env!("CARGO_PKG_VERSION"));
    write_state(&state);
    if update_available && env::var_os("DISPLAY").is_some() {
        let body = format!("Version {} is ready. Open Lemon Settings for details.", state.latest);
        let mut notification = Command::new("notify-send");
        notification.args(["--app-name=Lemon", "Lemon update available"]).arg(body)
            .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        #[cfg(unix)] {
            use std::os::unix::process::CommandExt;
            notification.process_group(0);
        }
        let _ = notification.spawn();
    }
    release_lock();
}

fn release_tag(json: &str) -> Option<String> {
    let key = json.find("\"tag_name\"")?;
    let after_key = json[key..].split_once(':')?.1.trim_start();
    let quoted = after_key.strip_prefix('"')?;
    let tag = quoted.split('"').next()?.trim();
    version_parts(tag)?;
    Some(tag.to_owned())
}
fn version_parts(value: &str) -> Option<[u64; 3]> {
    let mut parts = value.trim_start_matches('v').split('.');
    let parse = |part: &str| -> Option<u64> {
        let digits: String = part.chars().take_while(|character| character.is_ascii_digit()).collect();
        if digits.is_empty() { None } else { digits.parse().ok() }
    };
    Some([parse(parts.next()?)?, parse(parts.next()?)?, parse(parts.next()?)?])
}
pub fn is_newer(candidate: &str, current: &str) -> bool {
    match (version_parts(candidate), version_parts(current)) {
        (Some(candidate), Some(current)) => candidate > current,
        _ => false,
    }
}
pub fn status_text() -> String {
    let state = read_state();
    if state.checked_at == 0 { return "Update status: not checked yet".into(); }
    if state.failed { return "Update check failed; will retry later".into(); }
    let current = env!("CARGO_PKG_VERSION");
    if is_newer(&state.latest, current) { format!("Update available: {} (current {})", state.latest, current) }
    else { format!("Up to date ({current}) · latest {}", state.latest) }
}

#[cfg(test)]
mod tests {
    use super::{is_newer, release_tag};
    #[test]
    fn extracts_release_tag_without_a_json_dependency() {
        assert_eq!(release_tag(r#"{"tag_name": "v1.2.3", "name": "Release"}"#).as_deref(), Some("v1.2.3"));
        assert_eq!(release_tag(r#"{"tag_name":"not-a-version"}"#), None);
    }
    #[test]
    fn compares_numeric_versions_by_component() {
        assert!(is_newer("v0.10.0", "0.9.9"));
        assert!(!is_newer("v0.5.0", "0.5.0"));
        assert!(!is_newer("v0.4.9", "0.5.0"));
    }
}
