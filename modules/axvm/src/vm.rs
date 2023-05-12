use alloc::{sync::Arc, vec::Vec};
use axerrno::{AxError, AxResult};
use axhal::paging::PageTable;
use axsync::Mutex;

use crate::mmu::MemMap;
use crate::vcpu::VcpuState;

#[derive(Debug, Default)]
pub struct VmState {
    num_vcpus: u8,
    max_vcpus: u8,
    inner: Mutex<VmStateInner>,
}

#[derive(Debug, Default)]
pub(crate) struct VmStateInner {
    pub vcpus: Vec<Arc<VcpuState>>,
    pub memmap: Vec<MemMap>,
}

impl VmState {
    pub fn new(max_vcpus: u8) -> AxResult<Self> {
        // TODO:
        // let pgd_page = PageTable::try_new()?;

        Ok(Self {
            max_vcpus,
            ..Default::default()
        })
    }

    pub(crate) fn create_vcpu(&self, vcpu: Arc<VcpuState>) {
        self.inner.lock().vcpus.push(vcpu);
    }
}
