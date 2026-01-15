use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;
use walkdir::WalkDir;

fn copy_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let src_path = entry.path();
        let rel = src_path.strip_prefix(src).expect("relative path");
        let dest_path = dest.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(src_path, &dest_path)?;
        }
    }
    Ok(())
}

fn example_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name)
}

#[test]
fn cli_build_smoke() {
    let temp = tempdir().expect("temp dir");
    let pack_dir = temp.path().join("pack");
    copy_dir(&example_path("billing-demo"), &pack_dir).expect("copy example");

    let manifest_out = pack_dir.join("dist/manifest.cbor");
    let gtpack_out = pack_dir.join("dist/pack.gtpack");

    let output = std::process::Command::new(assert_cmd::cargo::cargo_bin!("greentic-pack"))
        .current_dir(&pack_dir)
        .env("GREENTIC_DIST_OFFLINE", "1")
        .args([
            "build",
            "--in",
            pack_dir.to_str().expect("pack dir"),
            "--manifest",
            manifest_out.to_str().expect("manifest path"),
            "--gtpack-out",
            gtpack_out.to_str().expect("gtpack path"),
        ])
        .output()
        .expect("run greentic-pack build");

    assert!(
        output.status.success(),
        "greentic-pack build failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(manifest_out.exists(), "manifest should be written");
    assert!(gtpack_out.exists(), "gtpack should be written");
}
