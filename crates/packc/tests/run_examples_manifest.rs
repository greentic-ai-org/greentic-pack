use assert_cmd::prelude::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn write_summary(path: &Path, flow: &str, nodes: &[(&str, serde_json::Value)]) {
    let summary_path = path.join(flow);
    if let Some(parent) = summary_path.parent() {
        fs::create_dir_all(parent).expect("create summary dir");
    }
    let nodes_json: serde_json::Map<_, _> = nodes
        .iter()
        .map(|(name, node)| ((*name).to_string(), node.clone()))
        .collect();
    let flow_name = summary_path.file_name().unwrap().to_string_lossy();
    let flow_name = flow_name
        .strip_suffix(".resolve.summary.json")
        .unwrap_or(&flow_name);
    let doc = json!({
        "schema_version": 1,
        "flow": flow_name,
        "nodes": nodes_json
    });
    fs::write(
        summary_path,
        serde_json::to_vec_pretty(&doc).expect("serialize summary"),
    )
    .expect("write summary");
}

fn digest_for(path: &Path) -> String {
    let bytes = fs::read(path).expect("read artifact for digest");
    format!("sha256:{:x}", Sha256::digest(bytes))
}

#[test]
fn build_all_examples_manifest_only() {
    let packs = [
        "examples/weather-demo",
        "examples/qa-demo",
        "examples/billing-demo",
        "examples/search-demo",
        "examples/reco-demo",
    ];

    for pack in packs {
        let pack_dir = workspace_root().join(pack);
        match pack {
            "examples/weather-demo" => {
                let summary_path = PathBuf::from("flows/weather_bot.ygtc.resolve.summary.json");
                let templating = pack_dir.join("components/templating.handlebars.wasm");
                write_summary(
                    &pack_dir,
                    summary_path.to_str().unwrap(),
                    &[
                        (
                            "collect_location",
                            json!({
                                "component_id": "qa.process",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.qa@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                        (
                            "forecast_weather",
                            json!({
                                "component_id": "mcp.exec",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.mcp@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                        (
                            "weather_text",
                            json!({
                                "component_id": "templating.handlebars",
                                "source": {"kind": "local", "path": "../components/templating.handlebars.wasm"},
                                "digest": digest_for(&templating)
                            }),
                        ),
                    ],
                );
            }
            "examples/qa-demo" => {
                write_summary(
                    &pack_dir,
                    "flows/qa_answer_flow.ygtc.resolve.summary.json",
                    &[
                        (
                            "call_llm",
                            json!({
                                "component_id": "llm.openai.chat",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.llm.openai@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                        (
                            "package_messages",
                            json!({
                                "component_id": "flow.return",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.flow@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                    ],
                );
                write_summary(
                    &pack_dir,
                    "flows/qa_orchestrator.ygtc.resolve.summary.json",
                    &[
                        (
                            "collect_question",
                            json!({
                                "component_id": "qa.process",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.qa@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                        (
                            "load_profile",
                            json!({
                                "component_id": "state.get",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.state@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                        (
                            "call_specialist",
                            json!({
                                "component_id": "flow.call",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.flow@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                        (
                            "respond_to_user",
                            json!({
                                "component_id": "messaging.emit",
                                "source": {"kind": "repo", "ref": "io.3bridges.components.messaging@1.0.0"},
                                "digest": "sha256:deadbeef"
                            }),
                        ),
                    ],
                );
            }
            "examples/billing-demo" => {
                let templating = pack_dir.join("components/templating.handlebars.wasm");
                write_summary(
                    &pack_dir,
                    "flows/main.ygtc.resolve.summary.json",
                    &[(
                        "start",
                        json!({
                            "component_id": "templating.handlebars",
                            "source": {"kind": "local", "path": "../components/templating.handlebars.wasm"},
                            "digest": digest_for(&templating)
                        }),
                    )],
                );
            }
            "examples/search-demo" => {
                let templating = pack_dir.join("components/templating.handlebars.wasm");
                write_summary(
                    &pack_dir,
                    "flows/main.ygtc.resolve.summary.json",
                    &[(
                        "start",
                        json!({
                            "component_id": "templating.handlebars",
                            "source": {"kind": "local", "path": "../components/templating.handlebars.wasm"},
                            "digest": digest_for(&templating)
                        }),
                    )],
                );
            }
            "examples/reco-demo" => {
                let templating = pack_dir.join("components/templating.handlebars.wasm");
                write_summary(
                    &pack_dir,
                    "flows/main.ygtc.resolve.summary.json",
                    &[(
                        "start",
                        json!({
                            "component_id": "templating.handlebars",
                            "source": {"kind": "local", "path": "../components/templating.handlebars.wasm"},
                            "digest": digest_for(&templating)
                        }),
                    )],
                );
            }
            _ => {}
        }
        let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("greentic-pack"));
        cmd.current_dir(workspace_root());
        cmd.args(["build", "--in", pack, "--dry-run", "--log", "warn"]);
        cmd.assert().success();
    }
}
