//! Where a Colony program keeps its files, on every platform.
//!
//! Every program used to work this out for itself, and they disagreed — three
//! different `dirs` calls for what was meant to be the same directory, which on
//! Linux all collapse to `~/.config` and hide the disagreement until someone
//! runs the program on Windows. This module answers it once.
//!
//! # The layout
//!
//! Everything a Colony program owns lives under a `Colony/<Program>/` pair: the
//! organisation, then the program. `<Program>` is the display name, capitalised
//! as the program spells it — `Colony`, `Digger`, `Grape`, `Eidos`.
//!
//! | Kind | Linux | Windows | macOS |
//! |---|---|---|---|
//! | [`config_dir`] | `~/.config/Colony/<P>/` | `%LOCALAPPDATA%\Colony\<P>\` | `~/Library/Application Support/Colony/<P>/` |
//! | [`data_dir`] | `~/.local/share/Colony/<P>/` | `%LOCALAPPDATA%\Colony\<P>\` | `~/Library/Application Support/Colony/<P>/` |
//! | [`cache_dir`] | `~/.cache/Colony/<P>/` | `%LOCALAPPDATA%\Colony\<P>\cache\` | `~/Library/Caches/Colony/<P>/` |
//!
//! On Windows and macOS, config and data are the **same directory** — those
//! platforms do not draw the distinction Linux does. Keep the two apart by
//! sub-directory (`preferences/`, `apps/`), never by relying on the roots
//! differing, or a file will silently collide on two platforms out of three.
//!
//! # Which one
//!
//! - [`config_dir`] — what the user chose, and what they would want to keep or
//!   copy to another machine: preferences, credentials, custom paths.
//! - [`data_dir`] — what the program produced and cannot re-derive: installed
//!   binaries, databases, history.
//! - [`cache_dir`] — what the program can rebuild by asking again. Deleting it
//!   must never lose anything but time.
//!
//! When in doubt: if losing it would annoy the user, it is not cache.

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

/// The organisation directory every Colony program nests under.
pub const VENDOR: &str = "Colony";

/// The shared install root for programs the launcher installs, a sibling of the
/// per-program directories rather than a child of the launcher's own.
///
/// Installed programs belong to the ecosystem, not to Colony-the-launcher: a
/// user uninstalling the launcher should not be told their programs lived
/// inside it.
pub const APPS: &str = "apps";

/// Reject anything that is not a single, safe path component.
///
/// The program name reaches these functions from a manifest or a config file in
/// the general case, and it is joined into a path that later gets written to
/// and removed. `..` or a separator here would let it escape the Colony tree.
fn component(name: &str) -> io::Result<&str> {
    let invalid = name.is_empty()
        || name == "."
        || name == ".."
        || name.contains(['/', '\\', '\0'])
        || Path::new(name).components().count() != 1
        || Path::new(name).file_name() != Some(OsStr::new(name));

    if invalid {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{name:?} is not a safe single path component"),
        ));
    }
    Ok(name)
}

fn missing(what: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::NotFound,
        format!("cannot determine the platform {what} directory"),
    )
}

fn ensure(path: PathBuf) -> io::Result<PathBuf> {
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

/// Paths without the side effect of creating them.
///
/// Use these to *show* a path — an About screen, a log line, a FAQ table.
/// Displaying where preferences would live should not bring the directory into
/// existence.
pub mod locate {
    use super::*;

    /// See [`super::config_dir`].
    pub fn config_dir(program: &str) -> io::Result<PathBuf> {
        Ok(dirs::config_local_dir()
            .ok_or_else(|| missing("config"))?
            .join(VENDOR)
            .join(component(program)?))
    }

    /// See [`super::data_dir`].
    pub fn data_dir(program: &str) -> io::Result<PathBuf> {
        Ok(dirs::data_local_dir()
            .ok_or_else(|| missing("data"))?
            .join(VENDOR)
            .join(component(program)?))
    }

    /// See [`super::cache_dir`].
    pub fn cache_dir(program: &str) -> io::Result<PathBuf> {
        Ok(dirs::cache_dir()
            .ok_or_else(|| missing("cache"))?
            .join(VENDOR)
            .join(component(program)?))
    }

    /// See [`super::apps_dir`].
    pub fn apps_dir() -> io::Result<PathBuf> {
        Ok(dirs::data_local_dir()
            .ok_or_else(|| missing("data"))?
            .join(VENDOR)
            .join(APPS))
    }

    /// See [`super::app_dir`].
    pub fn app_dir(repo: &str) -> io::Result<PathBuf> {
        Ok(apps_dir()?.join(component(repo)?))
    }
}

/// The program's configuration directory, created if it does not exist.
///
/// ```no_run
/// let prefs = colony_ui::paths::config_dir("Digger")?.join("preferences.json");
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn config_dir(program: &str) -> io::Result<PathBuf> {
    ensure(locate::config_dir(program)?)
}

/// The program's data directory, created if it does not exist.
pub fn data_dir(program: &str) -> io::Result<PathBuf> {
    ensure(locate::data_dir(program)?)
}

/// The program's cache directory, created if it does not exist.
pub fn cache_dir(program: &str) -> io::Result<PathBuf> {
    ensure(locate::cache_dir(program)?)
}

/// The shared install root, created if it does not exist.
pub fn apps_dir() -> io::Result<PathBuf> {
    ensure(locate::apps_dir()?)
}

/// One installed program's directory under [`apps_dir`], created if needed.
pub fn app_dir(repo: &str) -> io::Result<PathBuf> {
    ensure(locate::app_dir(repo)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_root_nests_under_the_vendor_then_the_program() {
        for path in [
            locate::config_dir("Digger").unwrap(),
            locate::data_dir("Digger").unwrap(),
            locate::cache_dir("Digger").unwrap(),
        ] {
            let tail: Vec<_> = path
                .components()
                .rev()
                .take(2)
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            assert_eq!(
                tail,
                vec!["Digger".to_string(), "Colony".to_string()],
                "{path:?}"
            );
        }
    }

    #[test]
    fn installed_programs_are_a_sibling_of_the_per_program_dirs() {
        // apps/ hangs off Colony/, NOT off Colony/Colony/ — an installed
        // program does not live inside the launcher.
        let apps = locate::apps_dir().unwrap();
        assert!(
            apps.ends_with("Colony/apps") || apps.ends_with("Colony\\apps"),
            "{apps:?}"
        );
        assert_eq!(locate::app_dir("Grape").unwrap(), apps.join("Grape"));
    }

    #[test]
    fn a_program_name_cannot_escape_the_colony_tree() {
        for bad in ["..", ".", "", "../../etc", "a/b", "a\\b", "with\0nul"] {
            assert!(
                locate::config_dir(bad).is_err(),
                "{bad:?} should be rejected as a program name"
            );
            assert!(
                locate::app_dir(bad).is_err(),
                "{bad:?} should be rejected as a repo name"
            );
        }
    }

    #[test]
    fn ordinary_program_names_are_accepted() {
        for good in ["Colony", "Digger", "Grape", "Eidos", "SAM - Colony Edition"] {
            assert!(
                locate::config_dir(good).is_ok(),
                "{good:?} should be accepted"
            );
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_uses_the_xdg_directories() {
        let home = std::env::var("HOME").expect("HOME");
        // XDG_* may be set in the environment; only assert the shape when it is
        // not, so the test says something on a default machine and stays quiet
        // on a customised one.
        if std::env::var_os("XDG_CONFIG_HOME").is_none() {
            assert_eq!(
                locate::config_dir("Colony").unwrap(),
                PathBuf::from(&home).join(".config/Colony/Colony")
            );
        }
        if std::env::var_os("XDG_DATA_HOME").is_none() {
            assert_eq!(
                locate::data_dir("Colony").unwrap(),
                PathBuf::from(&home).join(".local/share/Colony/Colony")
            );
            assert_eq!(
                locate::apps_dir().unwrap(),
                PathBuf::from(&home).join(".local/share/Colony/apps")
            );
        }
        if std::env::var_os("XDG_CACHE_HOME").is_none() {
            assert_eq!(
                locate::cache_dir("Colony").unwrap(),
                PathBuf::from(&home).join(".cache/Colony/Colony")
            );
        }
    }

    /// On Windows and macOS `config_local_dir` and `data_local_dir` are the same
    /// folder, so the two roots coincide. That is expected — it is why the
    /// layout separates config from data by sub-directory rather than by root.
    #[cfg(not(target_os = "linux"))]
    #[test]
    fn config_and_data_may_share_a_root_off_linux() {
        assert_eq!(
            locate::config_dir("Colony").unwrap(),
            locate::data_dir("Colony").unwrap()
        );
    }
}
