use alloc::vec::Vec;
use core::fmt::Error;

use dtb_walker::{Dtb, DtbObj, Property, WalkOperation as Op};
pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo, Error> {
    unsafe {
        let dtb = Dtb::from_raw_parts_unchecked(dtb_pa as *const u8);

        let mut memory_addr = 0;
        let mut memory_size = 0;
        let mut mmio_regions = Vec::new();

        dtb.walk(|path, obj| match obj {
            DtbObj::Property(Property::Reg(reg)) if path.last().starts_with(b"memory") => {
                for r in reg {
                    memory_addr = r.start;
                    memory_size = r.end - r.start;
                }
                Op::StepOver
            }
            DtbObj::Property(Property::Reg(reg)) if path.last().starts_with(b"virtio_mmio") => {
                for r in reg {
                    mmio_regions.push((r.start, r.end - r.start));
                }
                Op::StepOver
            }
            _ => Op::StepInto,
        });

        Ok(DtbInfo {
            memory_addr,
            memory_size,
            mmio_regions,
        })
    }
}
