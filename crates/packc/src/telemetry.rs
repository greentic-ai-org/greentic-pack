use anyhow::Result;
use greentic_config_types::{TelemetryConfig, TelemetryExporterKind};
pub use greentic_telemetry::with_task_local;
use greentic_telemetry::{
    TelemetryConfig as ServiceTelemetryConfig, TelemetryCtx,
    export::{ExportConfig, ExportMode, Sampling},
    init_telemetry_auto, init_telemetry_from_config, set_current_telemetry_ctx,
};
use greentic_types::TenantCtx;

/// Install the default Greentic telemetry stack for the given service.
pub fn install(service_name: &str) -> Result<()> {
    init_telemetry_auto(ServiceTelemetryConfig {
        service_name: service_name.to_string(),
    })
}

/// Install telemetry honoring greentic-config telemetry settings.
pub fn install_with_config(service_name: &str, cfg: &TelemetryConfig) -> Result<()> {
    if !cfg.enabled || matches!(cfg.exporter, TelemetryExporterKind::None) {
        return Ok(());
    }

    let export = match cfg.exporter {
        TelemetryExporterKind::Otlp => ExportConfig {
            mode: ExportMode::OtlpGrpc,
            endpoint: cfg.endpoint.clone(),
            headers: Default::default(),
            sampling: Sampling::TraceIdRatio(cfg.sampling as f64),
            compression: None,
        },
        TelemetryExporterKind::Stdout => ExportConfig {
            mode: ExportMode::JsonStdout,
            endpoint: None,
            headers: Default::default(),
            sampling: Sampling::TraceIdRatio(cfg.sampling as f64),
            compression: None,
        },
        TelemetryExporterKind::None => unreachable!("handled above"),
    };

    init_telemetry_from_config(
        ServiceTelemetryConfig {
            service_name: service_name.to_string(),
        },
        export,
    )
}

/// Map the provided tenant context into the task-local telemetry slot.
pub fn set_current_tenant_ctx(ctx: &TenantCtx) {
    let mut telemetry = TelemetryCtx::new(ctx.tenant_id.as_ref());

    if let Some(session) = ctx.session_id() {
        telemetry = telemetry.with_session(session);
    }
    if let Some(flow) = ctx.flow_id() {
        telemetry = telemetry.with_flow(flow);
    }
    if let Some(node) = ctx.node_id() {
        telemetry = telemetry.with_node(node);
    }
    if let Some(provider) = ctx.provider_id() {
        telemetry = telemetry.with_provider(provider);
    }

    set_current_telemetry_ctx(telemetry);
}
