# Security Fix Report

## Alerts Addressed
- GHSA-r6v5-fh4h-64xc (CVE-2026-25727) in `time` (runtime)
- GHSA-vc8c-j3xm-xj73 (CVE-2026-24116) in `wasmtime` (runtime)

## Changes Made
- Updated `time` in `examples/qa-demo/.packc/pack_component/Cargo.lock` from `0.3.44` to `0.3.47` (includes `time-core 0.1.8`, `time-macros 0.2.27`).
- Updated `wasmtime` in `examples/qa-demo/.packc/pack_component/Cargo.lock` from `38.0.4` to `41.0.3`, along with the matching `wasmtime-*` internal crates and required supporting crates (`pulley-*`, `winch-codegen`, `wasmparser 0.243.0`, `wasm-encoder 0.243.0`, `wasmprinter 0.243.0`, `wit-parser 0.243.0`, `wat 1.245.1`, `wast 245.0.1`, `wasm-compose 0.243.0`, `rustix 1.1.3`, `indexmap 2.13.0`).
- Disambiguated `indexmap` dependencies in the lockfile to avoid ambiguity after adding `indexmap 2.13.0`.

## Commands Run
- `RUSTUP_HOME=/tmp/rustup CARGO_HOME=/tmp/cargo RUSTUP_OFFLINE=1 RUSTUP_DISABLE_SELF_UPDATE=1 cargo test --locked` (failed: rustup attempted to download toolchain metadata due to network restrictions in CI)

## Remaining Alerts / Blockers
- None. Both listed alerts are addressed by lockfile updates.
- Testing is blocked in this CI environment because rustup attempts to reach `static.rust-lang.org` even with `RUSTUP_OFFLINE=1`.
