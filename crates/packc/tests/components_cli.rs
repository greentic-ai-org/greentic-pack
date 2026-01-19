use assert_cmd::prelude::*;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

#[test]
fn components_command_reports_counts_and_updates_manifest() {
    let temp = tempfile::tempdir().expect("temp dir");
    let pack_dir = temp.path().join("demo-pack");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("greentic-pack"));
    cmd.current_dir(workspace_root());
    cmd.args([
        "new",
        "demo-pack",
        "--dir",
        pack_dir.to_str().unwrap(),
        "--log",
        "warn",
    ]);
    cmd.assert().success();

    let components_dir = pack_dir.join("components");
    let nested_dir = components_dir.join("deep").join("wasm");
    fs::create_dir_all(&nested_dir).unwrap();
    fs::write(nested_dir.join("component.wasm"), [0x00, 0x61, 0x73, 0x6d]).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("greentic-pack"));
    cmd.current_dir(workspace_root());
    cmd.args([
        "components",
        "--in",
        pack_dir.to_str().unwrap(),
        "--json",
        "--log",
        "warn",
    ]);
    let output = cmd.output().expect("run packc components");
    assert!(
        output.status.success(),
        "components command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: JsonValue = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["components"]["total"], 1);

    let manifest: packc::config::PackConfig =
        serde_yaml_bw::from_str(&fs::read_to_string(pack_dir.join("pack.yaml")).unwrap()).unwrap();
    let ids: Vec<_> = manifest.components.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(ids, vec!["deep"]);
    let paths: Vec<_> = manifest
        .components
        .iter()
        .map(|c| c.wasm.to_string_lossy().to_string())
        .collect();
    assert_eq!(paths, vec!["components/deep/wasm/component.wasm"]);
}

#[test]
fn components_command_uses_manifest_id_when_available() {
    let temp = tempfile::tempdir().expect("temp dir");
    let pack_dir = temp.path().join("demo-pack");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("greentic-pack"));
    cmd.current_dir(workspace_root());
    cmd.args([
        "new",
        "demo-pack",
        "--dir",
        pack_dir.to_str().unwrap(),
        "--log",
        "warn",
    ]);
    cmd.assert().success();

    let components_dir = pack_dir.join("components").join("hello-world");
    fs::create_dir_all(&components_dir).unwrap();
    fs::write(
        components_dir.join("component.wasm"),
        [0x00, 0x61, 0x73, 0x6d],
    )
    .unwrap();
    let manifest = serde_json::json!({
        "id": "ai.greentic.hello-world",
        "version": "0.1.0",
        "supports": ["messaging"],
        "world": "greentic:component/component@0.5.0",
        "profiles": {
            "default": "stateless",
            "supported": ["stateless"]
        },
        "capabilities": {
            "wasi": {
                "random": false,
                "clocks": false
            },
            "host": {}
        },
        "operations": [
            {
                "name": "handle_message",
                "input_schema": {},
                "output_schema": {}
            }
        ]
    });
    fs::write(
        components_dir.join("component.manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("greentic-pack"));
    cmd.current_dir(workspace_root());
    cmd.args([
        "components",
        "--in",
        pack_dir.to_str().unwrap(),
        "--json",
        "--log",
        "warn",
    ]);
    let output = cmd.output().expect("run packc components");
    assert!(
        output.status.success(),
        "components command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest: packc::config::PackConfig =
        serde_yaml_bw::from_str(&fs::read_to_string(pack_dir.join("pack.yaml")).unwrap()).unwrap();
    let ids: Vec<_> = manifest.components.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(ids, vec!["ai.greentic.hello-world"]);
}
