#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use greentic_interfaces_host::state::greentic::state::state_store::{
    self as state_store_v1, Host as StateStoreHost,
};
use wasmtime::component::{HasSelf, Linker};

#[derive(Default)]
pub struct DescribeHostState;

impl StateStoreHost for DescribeHostState {
    fn read(
        &mut self,
        _key: state_store_v1::StateKey,
        _ctx: Option<state_store_v1::TenantCtx>,
    ) -> std::result::Result<Vec<u8>, state_store_v1::HostError> {
        Ok(Vec::new())
    }

    fn write(
        &mut self,
        _key: state_store_v1::StateKey,
        _bytes: Vec<u8>,
        _ctx: Option<state_store_v1::TenantCtx>,
    ) -> std::result::Result<state_store_v1::OpAck, state_store_v1::HostError> {
        Ok(state_store_v1::OpAck::Ok)
    }

    fn delete(
        &mut self,
        _key: state_store_v1::StateKey,
        _ctx: Option<state_store_v1::TenantCtx>,
    ) -> std::result::Result<state_store_v1::OpAck, state_store_v1::HostError> {
        Ok(state_store_v1::OpAck::Ok)
    }
}

pub fn add_describe_host_imports(linker: &mut Linker<DescribeHostState>) -> Result<()> {
    state_store_v1::add_to_linker::<DescribeHostState, HasSelf<DescribeHostState>>(
        linker,
        |host: &mut DescribeHostState| host,
    )
    .context("register state-store@1.0.0 describe host stub")?;
    Ok(())
}
