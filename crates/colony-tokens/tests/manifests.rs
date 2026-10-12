//! The example `colony.json` manifests are documentation, and documentation
//! that does not compile is worse than none — a program author copies one of
//! these, so each must be a manifest Colony would actually accept.

use std::collections::BTreeSet;
use std::path::PathBuf;

use colony_tokens::manifest::{Category, ColonyManifest, CATEGORIES, PLATFORMS};

fn examples_dir() -> PathBuf {
    colony_tokens::repo_root()
        .expect("repository root")
        .join("manifests/examples")
}

fn examples() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(examples_dir()).expect("manifests/examples exists") {
        let path = entry.expect("readable entry").path();
        if path.extension().is_some_and(|e| e == "json") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let src = std::fs::read_to_string(&path).expect("readable example");
            out.push((name, src));
        }
    }
    out.sort();
    assert!(!out.is_empty(), "no example manifests found");
    out
}

#[test]
fn every_example_parses_and_validates() {
    for (name, src) in examples() {
        let manifest: ColonyManifest = serde_json::from_str(&src)
            .unwrap_or_else(|e| panic!("{name}: does not parse as a colony.json: {e}"));
        manifest
            .validate()
            .unwrap_or_else(|e| panic!("{name}: {e:#}"));
    }
}

#[test]
fn every_example_points_at_the_generated_schema() {
    // An example without the $schema line teaches authors to omit it, which
    // costs them editor validation for free.
    for (name, src) in examples() {
        let manifest: ColonyManifest = serde_json::from_str(&src).expect("parses");
        let schema = manifest
            .schema
            .unwrap_or_else(|| panic!("{name}: missing the $schema pointer"));
        assert!(
            schema.ends_with("generated/colony.schema.json"),
            "{name}: $schema should point at generated/colony.schema.json, got {schema:?}"
        );
    }
}

#[test]
fn examples_cover_every_documented_shape() {
    // Each of these is a distinct path through Colony's installer. Losing
    // coverage of one means the next author has nothing to copy from.
    let mut saw_minimal = false;
    let mut saw_icon = false;
    let mut saw_explicit_file = false;
    let mut saw_pattern_archive = false;
    let mut saw_signed = false;

    for (_, src) in examples() {
        let m: ColonyManifest = serde_json::from_str(&src).expect("parses");
        saw_minimal |= m.platforms.is_empty() && m.release_files.is_empty() && m.icon.is_none();
        saw_icon |= m.icon.is_some();
        saw_signed |= m.signed;
        for entry in m.release_files.values() {
            saw_explicit_file |= entry.file.is_some();
            saw_pattern_archive |= entry.file_pattern.is_some() && entry.binary.is_some();
        }
    }

    assert!(
        saw_minimal,
        "no example of the minimal auto-detected manifest"
    );
    assert!(saw_icon, "no example declaring an icon");
    assert!(
        saw_explicit_file,
        "no example pinning exact asset filenames"
    );
    assert!(
        saw_pattern_archive,
        "no example resolving a versioned archive by pattern"
    );
    assert!(saw_signed, "no example requiring signed assets");
}

#[test]
fn the_generated_schema_agrees_with_the_types() {
    let root = colony_tokens::repo_root().expect("repository root");
    let src = std::fs::read_to_string(root.join("generated/colony.schema.json"))
        .expect("generated/colony.schema.json exists");
    let schema: serde_json::Value = serde_json::from_str(&src).expect("valid JSON");

    let enumerated: BTreeSet<String> = schema["properties"]["category"]["enum"]
        .as_array()
        .expect("category enum")
        .iter()
        .map(|v| v.as_str().expect("string").to_string())
        .collect();
    let expected: BTreeSet<String> = Category::all_spellings()
        .into_iter()
        .map(str::to_string)
        .collect();
    assert_eq!(enumerated, expected, "schema category enum drifted");

    let platform_keys: BTreeSet<String> = schema["properties"]["releaseFiles"]["properties"]
        .as_object()
        .expect("releaseFiles properties")
        .keys()
        .cloned()
        .collect();
    let expected: BTreeSet<String> = PLATFORMS.iter().map(|p| (*p).to_string()).collect();
    assert_eq!(platform_keys, expected, "schema platform keys drifted");

    // Guard the canonical spellings themselves: Colony files a program by these
    // strings, so silently renaming one would unfile every program using it.
    let canonical: Vec<&str> = CATEGORIES.iter().map(|c| c.canonical).collect();
    assert_eq!(
        canonical,
        vec![
            "development",
            "graphics",
            "network",
            "office",
            "multimedia",
            "system",
            "utility",
            "security",
            "game",
            "other",
        ]
    );
}
