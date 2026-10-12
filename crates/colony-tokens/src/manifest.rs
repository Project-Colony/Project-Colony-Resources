//! The `colony.json` contract.
//!
//! Every repository in the Project Colony organisation that wants to appear in
//! the launcher ships a `colony.json` at its root. Colony's catalog reads it to
//! decide what the program is called, where it belongs, and which release asset
//! to install on the running platform.
//!
//! These types mirror `ColonyManifest` in Colony's `src/github/types.rs`. They
//! live here so the contract is versioned in one place and the JSON Schema in
//! `generated/colony.schema.json` is derived rather than hand-written — the
//! same discipline the palettes get.

use std::collections::BTreeMap;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Where a program is filed in the launcher's category list.
///
/// Colony matches these case-insensitively after trimming, and warns-and-ignores
/// anything it does not recognize — an unknown category means the program is
/// simply never filtered into a section, which is easy to miss. Validate here
/// instead.
pub const CATEGORIES: &[Category] = &[
    Category::new("development", &[]),
    Category::new("graphics", &[]),
    Category::new("network", &[]),
    Category::new("office", &[]),
    Category::new("multimedia", &[]),
    Category::new("system", &[]),
    Category::new("utility", &["utilities"]),
    Category::new("security", &[]),
    Category::new("game", &["games"]),
    Category::new("other", &[]),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Category {
    /// The form to prefer in new manifests.
    pub canonical: &'static str,
    /// Spellings Colony also accepts.
    pub aliases: &'static [&'static str],
}

impl Category {
    const fn new(canonical: &'static str, aliases: &'static [&'static str]) -> Self {
        Category { canonical, aliases }
    }

    /// Resolve a manifest's `category` string the way Colony does.
    pub fn resolve(value: &str) -> Option<&'static Category> {
        let value = value.trim().to_ascii_lowercase();
        CATEGORIES
            .iter()
            .find(|c| c.canonical == value || c.aliases.contains(&value.as_str()))
    }

    /// Every accepted spelling, canonical first — what the schema enumerates.
    pub fn all_spellings() -> Vec<&'static str> {
        let mut out = Vec::new();
        for category in CATEGORIES {
            out.push(category.canonical);
            out.extend_from_slice(category.aliases);
        }
        out
    }
}

/// The platform keys Colony resolves at runtime.
///
/// `macos` is Apple Silicon and `macos-x86` is Intel; Colony picks between them
/// with `cfg!(target_arch)`, so a program shipping only `macos` is simply
/// unavailable to Intel Macs rather than broken on them.
pub const PLATFORMS: &[&str] = &["linux", "windows", "macos", "macos-x86"];

/// The release-asset suffix Colony expects per platform, used for the
/// auto-detection path. Order matters: `macos-x86` must be tested before
/// `macos` or the longer name matches the shorter rule.
pub const PLATFORM_ASSET_SUFFIXES: &[(&str, &str)] = &[
    ("linux", "-linux"),
    ("windows", "-windows.exe"),
    ("macos-x86", "-macos-x86"),
    ("macos", "-macos"),
];

/// Which asset to fetch for one platform, and how to treat it once downloaded.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReleaseFileEntry {
    /// A git tag, or the literal `"latest"` to always track the newest release.
    pub tag: String,
    /// Exact asset filename. Mutually exclusive with `file_pattern`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Case-insensitive pattern matched against the release's asset names, for
    /// assets whose name carries the version. Must match exactly one asset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_pattern: Option<String>,
    /// Binary to extract from inside the downloaded archive. When absent, the
    /// downloaded file *is* the binary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    /// Optional SHA-256 of the asset, as 64 lowercase hex characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

/// A repository's `colony.json`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColonyManifest {
    /// Optional pointer to `generated/colony.schema.json`, so editors validate
    /// the file as it is typed. Colony ignores it.
    ///
    /// Everything *else* unknown is rejected: Colony silently drops fields it
    /// does not recognize, which turns a typo into a setting that quietly never
    /// applies. Being stricter here is the point of validating at all.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Display name in the launcher. Free-form — this is the only field the
    /// user actually reads, so it may differ from the repository name.
    pub name: String,
    pub category: String,
    /// Platforms this program ships for. Leave empty to let Colony auto-detect
    /// them from the release asset names, which is the recommended path when
    /// the assets follow the naming convention.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<String>,
    /// Explicit per-platform asset resolution. Only needed when the assets do
    /// *not* follow the naming convention, or when installing from an archive.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub release_files: BTreeMap<String, ReleaseFileEntry>,
    /// Repo-relative path to a square PNG icon. When absent, Colony probes
    /// `icon.png` at the repo root, then falls back to a tinted category
    /// hexagon coloured by `app_tint(name)`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Require a valid detached ed25519 signature for every asset. When true, a
    /// missing or invalid `<asset>.sig` aborts the install instead of falling
    /// back to the unsigned path.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub signed: bool,
}

impl ColonyManifest {
    /// Everything Colony would either reject outright or silently ignore.
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            bail!("`name` must not be empty");
        }

        if Category::resolve(&self.category).is_none() {
            bail!(
                "unknown category {:?} — Colony ignores it and the program is \
                 filed nowhere. Valid: {}",
                self.category,
                CATEGORIES
                    .iter()
                    .map(|c| c.canonical)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        for platform in &self.platforms {
            if !PLATFORMS.contains(&platform.as_str()) {
                bail!(
                    "unknown platform {platform:?} — valid: {}",
                    PLATFORMS.join(", ")
                );
            }
        }

        for (platform, entry) in &self.release_files {
            if !PLATFORMS.contains(&platform.as_str()) {
                bail!(
                    "releaseFiles has an unknown platform key {platform:?} — valid: {}",
                    PLATFORMS.join(", ")
                );
            }
            entry.validate(platform)?;
        }

        // Declaring a platform with no way to resolve its asset is only safe
        // when auto-detection can do the job, i.e. when no releaseFiles were
        // given at all. A partial map is almost always an oversight.
        if !self.release_files.is_empty() {
            for platform in &self.platforms {
                if !self.release_files.contains_key(platform) {
                    bail!(
                        "platform {platform:?} is declared but has no releaseFiles \
                         entry; either add one or drop releaseFiles entirely and \
                         let Colony auto-detect from the asset names"
                    );
                }
            }
        }

        if let Some(icon) = &self.icon {
            if !icon.to_ascii_lowercase().ends_with(".png") {
                bail!("`icon` must point at a PNG, got {icon:?}");
            }
            if icon.starts_with('/') || icon.contains("..") {
                bail!("`icon` must be a repo-relative path without `..`, got {icon:?}");
            }
        }

        Ok(())
    }

    /// The asset name Colony's auto-detection expects for a repository.
    pub fn conventional_asset(repo_name: &str, platform: &str) -> Option<String> {
        PLATFORM_ASSET_SUFFIXES
            .iter()
            .find(|(key, _)| *key == platform)
            .map(|(_, suffix)| format!("{}{suffix}", repo_name.to_ascii_lowercase()))
    }
}

impl ReleaseFileEntry {
    fn validate(&self, platform: &str) -> Result<()> {
        if self.tag.trim().is_empty() {
            bail!(
                "{platform}: `tag` must not be empty (use \"latest\" to track the newest release)"
            );
        }
        match (&self.file, &self.file_pattern) {
            (None, None) => bail!("{platform}: needs either `file` or `filePattern`"),
            (Some(_), Some(_)) => {
                bail!("{platform}: `file` and `filePattern` are mutually exclusive")
            }
            _ => {}
        }
        if let Some(sha) = &self.sha256 {
            if sha.len() != 64 || !sha.chars().all(|c| c.is_ascii_hexdigit()) {
                bail!("{platform}: `sha256` must be 64 hex characters");
            }
            if sha.chars().any(|c| c.is_ascii_uppercase()) {
                bail!("{platform}: `sha256` must be lowercase");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_categories_the_way_colony_does() {
        assert_eq!(Category::resolve("System").unwrap().canonical, "system");
        assert_eq!(Category::resolve("  Games ").unwrap().canonical, "game");
        assert_eq!(Category::resolve("Utilities").unwrap().canonical, "utility");
        assert!(Category::resolve("Nonsense").is_none());
    }

    #[test]
    fn builds_conventional_asset_names() {
        assert_eq!(
            ColonyManifest::conventional_asset("D1Gg2r", "windows").unwrap(),
            "d1gg2r-windows.exe"
        );
        assert_eq!(
            ColonyManifest::conventional_asset("Eidos", "macos-x86").unwrap(),
            "eidos-macos-x86"
        );
        assert!(ColonyManifest::conventional_asset("Eidos", "haiku").is_none());
    }

    fn minimal() -> ColonyManifest {
        ColonyManifest {
            schema: None,
            name: "Eidos".into(),
            category: "system".into(),
            platforms: Vec::new(),
            release_files: BTreeMap::new(),
            icon: None,
            signed: false,
        }
    }

    #[test]
    fn accepts_a_minimal_manifest() {
        minimal().validate().unwrap();
    }

    #[test]
    fn rejects_an_unknown_category() {
        let mut m = minimal();
        m.category = "Productivity".into();
        assert!(m
            .validate()
            .unwrap_err()
            .to_string()
            .contains("unknown category"));
    }

    #[test]
    fn rejects_a_platform_with_no_release_entry() {
        let mut m = minimal();
        m.platforms = vec!["linux".into(), "windows".into()];
        m.release_files.insert(
            "linux".into(),
            ReleaseFileEntry {
                tag: "latest".into(),
                file: Some("eidos-linux".into()),
                file_pattern: None,
                binary: None,
                sha256: None,
            },
        );
        assert!(m.validate().unwrap_err().to_string().contains("windows"));
    }

    #[test]
    fn rejects_file_and_pattern_together() {
        let mut m = minimal();
        m.release_files.insert(
            "linux".into(),
            ReleaseFileEntry {
                tag: "latest".into(),
                file: Some("eidos-linux".into()),
                file_pattern: Some("*-linux".into()),
                binary: None,
                sha256: None,
            },
        );
        assert!(m
            .validate()
            .unwrap_err()
            .to_string()
            .contains("mutually exclusive"));
    }

    #[test]
    fn rejects_a_malformed_checksum() {
        let mut m = minimal();
        m.release_files.insert(
            "linux".into(),
            ReleaseFileEntry {
                tag: "v1.0.0".into(),
                file: Some("eidos-linux".into()),
                file_pattern: None,
                binary: None,
                sha256: Some("ABC123".into()),
            },
        );
        assert!(m.validate().unwrap_err().to_string().contains("64 hex"));
    }
}
