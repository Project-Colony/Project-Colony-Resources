//! Reads `tokens/` — the source of truth — and produces everything in `generated/`.
//!
//! Nothing in this crate ships to users. It exists so that a colour is written
//! down exactly once, in `tokens/families/<family>.toml`, and every consumer
//! (Rust launcher, Discord CSS, website, docs) reads a generated artifact
//! instead of re-deriving it from someone else's source file.

pub mod color;
pub mod emit;
pub mod model;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use model::Tokens;

/// One file the generator owns, start to finish.
pub struct Artifact {
    /// Path relative to the repository root.
    pub path: PathBuf,
    pub contents: String,
}

/// Build every artifact from the loaded tokens. Pure — no I/O, so `check` and
/// `generate` cannot disagree about what the output should be.
pub fn plan(tokens: &Tokens) -> Result<Vec<Artifact>> {
    let mut out = Vec::new();

    out.push(Artifact {
        path: PathBuf::from("generated/rust/palettes.rs"),
        contents: emit::rust::render(tokens),
    });

    out.push(Artifact {
        path: PathBuf::from("generated/themes.json"),
        contents: to_json(&emit::json::render(tokens)),
    });

    for (family, variant) in tokens.variants() {
        out.push(Artifact {
            path: PathBuf::from(format!("generated/css/colony-{}.css", variant.slug(family))),
            contents: emit::css::render_variant(family, variant),
        });
    }
    out.push(Artifact {
        path: PathBuf::from("generated/css/colony-themes.css"),
        contents: emit::css::render_bundle(tokens),
    });
    out.push(Artifact {
        path: PathBuf::from("generated/css/colony-accents.css"),
        contents: emit::css::render_accents(tokens),
    });

    out.push(Artifact {
        path: PathBuf::from("generated/palette.schema.json"),
        contents: to_json(&emit::schema::render()),
    });

    let locales = emit::i18n::render(tokens)?;
    out.push(Artifact {
        path: PathBuf::from("generated/i18n/labels.fr.json"),
        contents: to_json(&locales.fr),
    });
    out.push(Artifact {
        path: PathBuf::from("generated/i18n/labels.en.json"),
        contents: to_json(&locales.en),
    });

    Ok(out)
}

fn to_json(value: &serde_json::Value) -> String {
    let mut s = serde_json::to_string_pretty(value).expect("json serialization is infallible");
    s.push('\n');
    s
}

/// Write every artifact and delete anything else living under `generated/`, so a
/// removed theme cannot leave an orphan stylesheet behind.
pub fn generate(repo_root: &Path) -> Result<Report> {
    let tokens = Tokens::load(&repo_root.join("tokens"))?;
    let artifacts = plan(&tokens)?;

    let mut report = Report::new(&tokens);
    let expected: BTreeSet<PathBuf> = artifacts.iter().map(|a| a.path.clone()).collect();

    for artifact in &artifacts {
        let full = repo_root.join(&artifact.path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let unchanged = std::fs::read_to_string(&full)
            .map(|existing| existing == artifact.contents)
            .unwrap_or(false);
        if unchanged {
            report.unchanged += 1;
        } else {
            std::fs::write(&full, &artifact.contents)
                .with_context(|| format!("writing {}", full.display()))?;
            report.written.push(artifact.path.clone());
        }
    }

    for stale in existing_files(&repo_root.join("generated"))? {
        let rel = stale
            .strip_prefix(repo_root)
            .unwrap_or(&stale)
            .to_path_buf();
        if !expected.contains(&rel) {
            std::fs::remove_file(&stale)
                .with_context(|| format!("removing stale {}", stale.display()))?;
            report.removed.push(rel);
        }
    }

    Ok(report)
}

/// Verify `generated/` matches `tokens/` without touching the working tree.
/// This is what CI runs; a non-empty result means someone edited a generated
/// file by hand or forgot to regenerate.
pub fn check(repo_root: &Path) -> Result<Vec<String>> {
    let tokens = Tokens::load(&repo_root.join("tokens"))?;
    let artifacts = plan(&tokens)?;
    let expected: BTreeSet<PathBuf> = artifacts.iter().map(|a| a.path.clone()).collect();

    let mut problems = Vec::new();
    for artifact in &artifacts {
        let full = repo_root.join(&artifact.path);
        match std::fs::read_to_string(&full) {
            Err(_) => problems.push(format!("missing: {}", artifact.path.display())),
            Ok(actual) if actual != artifact.contents => {
                problems.push(format!("out of date: {}", artifact.path.display()))
            }
            Ok(_) => {}
        }
    }
    for stale in existing_files(&repo_root.join("generated"))? {
        let rel = stale
            .strip_prefix(repo_root)
            .unwrap_or(&stale)
            .to_path_buf();
        if !expected.contains(&rel) {
            problems.push(format!("stale, not produced by tokens/: {}", rel.display()));
        }
    }
    Ok(problems)
}

fn existing_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).with_context(|| format!("reading {}", d.display()))? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

pub struct Report {
    pub families: usize,
    pub variants: usize,
    pub written: Vec<PathBuf>,
    pub removed: Vec<PathBuf>,
    pub unchanged: usize,
}

impl Report {
    fn new(tokens: &Tokens) -> Self {
        Report {
            families: tokens.families.len(),
            variants: tokens.variant_count(),
            written: Vec::new(),
            removed: Vec::new(),
            unchanged: 0,
        }
    }
}

/// Locate the repository root from the crate's own manifest path, so the tool
/// works from any working directory.
pub fn repo_root() -> Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest
        .ancestors()
        .nth(2)
        .context("cannot locate the repository root above tools/colony-tokens")?
        .to_path_buf();
    if !root.join("tokens").is_dir() {
        bail!("{} does not contain a tokens/ directory", root.display());
    }
    Ok(root)
}
