//! Wrappers and helper functions for guest physical memory addresses.

use core::convert::{From, Into};
use core::fmt;
use core::format_args;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use memory_addr::{align_down, align_offset, align_up, is_aligned, PAGE_SIZE_4K};

/// A guest physical memory address.
///
/// It's a wrapper type around an `usize`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct GuestPhysAddr(usize);

impl GuestPhysAddr {
    /// Converts an `usize` to a guest physical address.
    #[inline]
    pub const fn from(addr: usize) -> Self {
        Self(addr)
    }

    /// Converts the address to an `usize`.
    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Aligns the address downwards to the given alignment.
    ///
    /// See the [`align_down`] function for more information.
    #[inline]
    pub fn align_down<U>(self, align: U) -> Self
    where
        U: Into<usize>,
    {
        Self(align_down(self.0, align.into()))
    }

    /// Aligns the address upwards to the given alignment.
    ///
    /// See the [`align_up`] function for more information.
    #[inline]
    pub fn align_up<U>(self, align: U) -> Self
    where
        U: Into<usize>,
    {
        Self(align_up(self.0, align.into()))
    }

    /// Returns the offset of the address within the given alignment.
    ///
    /// See the [`align_offset`] function for more information.
    #[inline]
    pub fn align_offset<U>(self, align: U) -> usize
    where
        U: Into<usize>,
    {
        align_offset(self.0, align.into())
    }

    /// Checks whether the address has the demanded alignment.
    ///
    /// See the [`is_aligned`] function for more information.
    #[inline]
    pub fn is_aligned<U>(self, align: U) -> bool
    where
        U: Into<usize>,
    {
        is_aligned(self.0, align.into())
    }

    /// Aligns the address downwards to 4096 (bytes).
    #[inline]
    pub fn align_down_4k(self) -> Self {
        self.align_down(PAGE_SIZE_4K)
    }

    /// Aligns the address upwards to 4096 (bytes).
    #[inline]
    pub fn align_up_4k(self) -> Self {
        self.align_up(PAGE_SIZE_4K)
    }

    /// Returns the offset of the address within a 4K-sized page.
    #[inline]
    pub fn align_offset_4k(self) -> usize {
        self.align_offset(PAGE_SIZE_4K)
    }

    /// Checks whether the address is 4K-aligned.
    #[inline]
    pub fn is_aligned_4k(self) -> bool {
        self.is_aligned(PAGE_SIZE_4K)
    }
}

impl From<usize> for GuestPhysAddr {
    #[inline]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl From<GuestPhysAddr> for usize {
    #[inline]
    fn from(addr: GuestPhysAddr) -> usize {
        addr.0
    }
}

impl Add<usize> for GuestPhysAddr {
    type Output = Self;
    #[inline]
    fn add(self, rhs: usize) -> Self {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for GuestPhysAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl Sub<usize> for GuestPhysAddr {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: usize) -> Self {
        Self(self.0 - rhs)
    }
}

impl SubAssign<usize> for GuestPhysAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}

impl fmt::Debug for GuestPhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl fmt::LowerHex for GuestPhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl fmt::UpperHex for GuestPhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#X}", self.0))
    }
}
