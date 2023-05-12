use alloc::vec::Vec;
use axerrno::{AxError, AxResult};
use axhal::paging::PageTable;
use axtask::WaitQueue;
use memory_addr::PhysAddr;

use crate::mmu::{MemEvent, MemMap};
use crate::vcpu::{Vcpu, VcpuState};

pub struct VmState {
    num_vcpus: u8,
    max_vcpus: u8,
    pgd_phys: PhysAddr,
    event_mgr: Option<VmEventManager>,
    vcpus: Vec<Vcpu>,
    memmap: Vec<MemMap>,
    pt: PageTable,
}

pub(crate) struct VmEventManager {
    wait_queue: WaitQueue,
    events: Vec<MemEvent>,
}

impl VmState {
    pub fn new(max_vcpus: u8) -> AxResult<Self> {
        let pt = PageTable::try_new().map_err(|_| AxError::NoMemory)?;
        Ok(Self {
            num_vcpus: 0,
            max_vcpus,
            pgd_phys: pt.root_paddr(),
            event_mgr: None,
            vcpus: Vec::new(),
            memmap: Vec::new(),
            pt,
        })
    }

    pub fn create_vcpu(&mut self, id: u8) -> Vcpu {
        let vcpu = VcpuState::new(id);
        self.vcpus.push(vcpu.clone());
        vcpu
    }

    pub fn create_memmap(&mut self, memmap: MemMap) {
        self.memmap.push(memmap);
        self.memmap.sort_by_key(|m| m.gpa.as_usize());
    }

    pub fn register_event(&mut self, event: MemEvent) {
        if self.event_mgr.is_none() {
            self.event_mgr = Some(VmEventManager {
                wait_queue: WaitQueue::new(),
                events: Vec::new(),
            });
        }

        let mgr = self.event_mgr.as_mut().unwrap();
        mgr.events.push(event);
        mgr.events.sort_by_key(|e| e.gpa.as_usize());
    }

    pub fn wait_event(&self) {
        if let Some(mgr) = &self.event_mgr {
            mgr.wait_queue.wait();
        }
    }
}
