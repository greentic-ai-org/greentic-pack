router_echo component fixture

This component exports `wasix:mcp/router@25.6.18` with a single `echo` tool. It was built with `cargo component build --release` using the WIT in `wit/mcp-router.wit`.

Rebuild command (from this directory):

```
CARGO_NET_OFFLINE=true cargo component build --release
cp target/wasm32-wasip1/release/router_echo.wasm ../fixtures/router-echo-component.wasm
```
