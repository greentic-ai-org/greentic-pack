#![forbid(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use greentic_config::explain;

#[derive(Debug, Clone, Parser)]
pub struct ConfigArgs {}

pub fn handle(
    _args: ConfigArgs,
    json: bool,
    runtime: &crate::runtime::RuntimeContext,
) -> Result<()> {
    let report = explain(
        &runtime.resolved.config,
        &runtime.resolved.provenance,
        &runtime.resolved.warnings,
    );
    if json {
        println!("{}", serde_json::to_string_pretty(&report.json)?);
    } else {
        println!("{}", report.text);
    }
    Ok(())
}
