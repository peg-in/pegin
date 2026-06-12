//! Injects `PEGIN_JWT_HKDF_SALT` at compile time for wasm targets.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=PEGIN_JWT_HKDF_SALT");
    println!("cargo:rerun-if-changed=../pegin-wasm/.env");
    println!("cargo:rerun-if-changed=../../.env");

    let salt = resolve_salt().unwrap_or_else(|| {
        let mut msg = String::from(
            "PEGIN_JWT_HKDF_SALT must be set.\n\
             Export the env var or add PEGIN_JWT_HKDF_SALT=... to crates/pegin-wasm/.env\n\n\
             Checked:\n",
        );
        for path in env_file_paths() {
            let status = if path.is_file() {
                if read_env_key(&path, "PEGIN_JWT_HKDF_SALT").is_some() {
                    "found key"
                } else {
                    "file exists, key missing"
                }
            } else {
                "file not found"
            };
            let _ = writeln!(msg, "  - {} ({status})", path.display());
        }
        panic!("{msg}");
    });

    assert!(!salt.is_empty(), "PEGIN_JWT_HKDF_SALT must not be empty");

    println!("cargo:rustc-env=PEGIN_JWT_HKDF_SALT={salt}");
}

fn resolve_salt() -> Option<String> {
    if let Ok(value) = std::env::var("PEGIN_JWT_HKDF_SALT") {
        return Some(value);
    }

    for path in env_file_paths() {
        if let Some(value) = read_env_key(&path, "PEGIN_JWT_HKDF_SALT") {
            return Some(value);
        }
    }

    None
}

fn env_file_paths() -> Vec<PathBuf> {
    // Cargo always sets CARGO_MANIFEST_DIR for build scripts; read it at compile time.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vec![
        manifest.join("../pegin-wasm/.env"),
        manifest.join("../../.env"),
    ]
}

fn read_env_key(path: &Path, key: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let mut line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("export ") {
            line = rest.trim();
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        if name.trim() == key {
            return Some(trim_env_value(value.trim()));
        }
    }
    None
}

fn trim_env_value(value: &str) -> String {
    let value = value.trim();
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        let quote = bytes[0];
        if (quote == b'"' || quote == b'\'') && bytes[bytes.len() - 1] == quote {
            return value[1..value.len() - 1].to_owned();
        }
    }
    value.to_owned()
}
