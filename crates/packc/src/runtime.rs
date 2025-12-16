#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use greentic_config::{ConfigLayer, ConfigResolver, ResolvedConfig};
use greentic_types::ConnectionKind;
use std::sync::Arc;

pub struct RuntimeState {
    pub resolved: ResolvedConfig,
}

pub type RuntimeContext = Arc<RuntimeState>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkPolicy {
    Online,
    Offline,
}

impl RuntimeState {
    pub fn cache_dir(&self) -> PathBuf {
        self.resolved.config.paths.cache_dir.clone()
    }

    pub fn network_policy(&self) -> NetworkPolicy {
        if matches!(
            self.resolved.config.environment.connection,
            Some(ConnectionKind::Offline)
        ) {
            NetworkPolicy::Offline
        } else {
            NetworkPolicy::Online
        }
    }

    pub fn require_online(&self, action: &str) -> Result<()> {
        match self.network_policy() {
            NetworkPolicy::Online => Ok(()),
            NetworkPolicy::Offline => Err(anyhow::anyhow!(
                "network operation blocked in offline mode: {}",
                action
            )),
        }
    }

    pub fn warnings(&self) -> &[String] {
        &self.resolved.warnings
    }
}

pub fn resolve_runtime(
    project_root: Option<&Path>,
    cli_cache_dir: Option<&Path>,
    cli_offline: bool,
    cli_override: Option<&Path>,
) -> Result<RuntimeContext> {
    let mut resolver = ConfigResolver::new();
    if let Some(root) = project_root {
        resolver = resolver.with_project_root(root.to_path_buf());
    }

    if let Some(path) = cli_override {
        let layer = load_cli_override_layer(path)?;
        resolver = resolver.with_cli_overrides(layer);
    }

    let mut resolved = resolver.load()?;

    if cli_offline {
        resolved.config.environment.connection = Some(ConnectionKind::Offline);
        resolved
            .warnings
            .push("offline forced by CLI --offline flag".to_string());
    }

    if let Some(cache_dir) = cli_cache_dir {
        resolved.config.paths.cache_dir = cache_dir.to_path_buf();
    }

    Ok(Arc::new(RuntimeState { resolved }))
}

fn load_cli_override_layer(path: &Path) -> Result<ConfigLayer> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read config override {}", path.display()))?;
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default();
    let layer = if ext.eq_ignore_ascii_case("json") {
        serde_json::from_str(&contents)
            .with_context(|| format!("{} is not valid JSON", path.display()))?
    } else {
        toml::from_str(&contents)
            .with_context(|| format!("{} is not valid TOML", path.display()))?
    };
    Ok(layer)
}
