use axalloc::global_allocator;
use axerrno::{AxError, AxResult};
pub use memory_addr::{is_aligned_4k, VirtAddr, PAGE_SIZE_4K};
pub use page_table_entry::MappingFlags;

pub use crate::types::GuestPhysAddr;

const PAGE_SIZE: usize = PAGE_SIZE_4K;

pub struct MemMap {
    pub hva: VirtAddr,
    pub gpa: GuestPhysAddr,
    pub size: usize,
    pub flags: MappingFlags,
}

pub struct MemEvent {
    pub gpa: GuestPhysAddr,
    pub size: usize,
}

impl MemMap {
    pub fn alloc(gpa: GuestPhysAddr, size: usize, flags: MappingFlags) -> AxResult<MemMap> {
        if !is_aligned_4k(size) {
            return Err(AxError::InvalidInput);
        }

        let vaddr = global_allocator()
            .alloc_pages(size / PAGE_SIZE, PAGE_SIZE)
            .map_err(|_| AxError::NoMemory)?;

        Ok(Self {
            hva: VirtAddr::from(vaddr),
            gpa,
            size,
            flags,
        })
    }
}
