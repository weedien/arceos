use memory_addr::VirtAddr;

use crate::types::GuestPhysAddr;

#[derive(Debug, Default)]
pub struct MemMap {
    pub hva: VirtAddr,
    pub gpa: GuestPhysAddr,
    pub size: usize,
    pub flags: u8,
}
