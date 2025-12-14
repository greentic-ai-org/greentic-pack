#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Cli,
    Env,
    Default,
}

#[derive(Debug, Clone)]
pub struct ProvenanceEntry {
    pub key: &'static str,
    pub value: String,
    pub source: Source,
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeWarnings {
    pub messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedRuntime {
    pub cache_dir: PathBuf,
    pub offline: bool,
    pub provenance: Vec<ProvenanceEntry>,
    pub warnings: RuntimeWarnings,
}

impl ResolvedRuntime {
    pub fn network_policy(&self) -> NetworkPolicy {
        if self.offline {
            NetworkPolicy::Offline
        } else {
            NetworkPolicy::Online
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkPolicy {
    Online,
    Offline,
}

impl NetworkPolicy {
    pub fn require_online(&self, action: &str) -> Result<()> {
        match self {
            NetworkPolicy::Online => Ok(()),
            NetworkPolicy::Offline => Err(anyhow::anyhow!(
                "network operation blocked in offline mode: {}",
                action
            )),
        }
    }

    pub fn is_offline(&self) -> bool {
        matches!(self, NetworkPolicy::Offline)
    }
}

pub fn resolve_runtime(
    pack_dir: &Path,
    cli_cache_dir: Option<&Path>,
    cli_offline: bool,
) -> ResolvedRuntime {
    let mut provenance = Vec::new();
    let mut warnings = RuntimeWarnings::default();

    let cache_dir = resolve_cache_dir(pack_dir, cli_cache_dir, &mut provenance);
    let offline = resolve_offline(cli_offline, &mut provenance, &mut warnings);

    ResolvedRuntime {
        cache_dir,
        offline,
        provenance,
        warnings,
    }
}

fn resolve_cache_dir(
    pack_dir: &Path,
    cli_cache_dir: Option<&Path>,
    provenance: &mut Vec<ProvenanceEntry>,
) -> PathBuf {
    if let Some(cli) = cli_cache_dir {
        provenance.push(ProvenanceEntry {
            key: "cache_dir",
            value: cli.display().to_string(),
            source: Source::Cli,
        });
        return cli.to_path_buf();
    }

    if let Ok(env_val) = std::env::var("GREENTIC_PACK_CACHE_DIR") {
        let path = PathBuf::from(env_val);
        provenance.push(ProvenanceEntry {
            key: "cache_dir",
            value: path.display().to_string(),
            source: Source::Env,
        });
        return path;
    }

    let default = pack_dir.join(".packc");
    provenance.push(ProvenanceEntry {
        key: "cache_dir",
        value: default.display().to_string(),
        source: Source::Default,
    });
    default
}

fn resolve_offline(
    cli_offline: bool,
    provenance: &mut Vec<ProvenanceEntry>,
    warnings: &mut RuntimeWarnings,
) -> bool {
    let env_offline = std::env::var("GREENTIC_PACK_OFFLINE")
        .ok()
        .and_then(|v| parse_bool(&v));

    let mut offline = false;
    let mut source = Source::Default;

    if let Some(env) = env_offline {
        offline = env;
        source = Source::Env;
    }

    if cli_offline {
        if offline && source == Source::Env {
            warnings.messages.push(
                "offline forced by CLI --offline flag (overrides GREENTIC_PACK_OFFLINE)".into(),
            );
        }
        offline = true;
        source = Source::Cli;
    }

    provenance.push(ProvenanceEntry {
        key: "offline",
        value: offline.to_string(),
        source,
    });

    offline
}

fn parse_bool(raw: &str) -> Option<bool> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}
