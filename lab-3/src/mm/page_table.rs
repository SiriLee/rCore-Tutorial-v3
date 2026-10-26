use core::ops::BitOr;

use super::{frame_alloc, FrameTracker, PhysPageNum, VirtPageNum};

#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct PageTableEntry(pub usize);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PteFlags(pub usize);

impl PteFlags {
    pub const V: Self = Self(1 << 0);
    pub const R: Self = Self(1 << 1);
    pub const W: Self = Self(1 << 2);
    pub const X: Self = Self(1 << 3);
    pub const U: Self = Self(1 << 4);

    pub const fn bits(self) -> usize {
        self.0
    }

    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    pub const fn intersects(self, flags: Self) -> bool {
        self.0 & flags.0 != 0
    }
}

impl BitOr for PteFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn new(ppn: PhysPageNum, flags: PteFlags) -> Self {
        Self((ppn.0 << 10) | flags.bits())
    }

    pub fn ppn(self) -> PhysPageNum {
        PhysPageNum(self.0 >> 10)
    }

    pub fn flags(self) -> PteFlags {
        PteFlags(self.0 & 0x3ff)
    }

    pub fn is_valid(self) -> bool {
        self.0 & PteFlags::V.bits() != 0
    }

    pub fn is_leaf(self) -> bool {
        self.0 & (PteFlags::R.bits() | PteFlags::W.bits() | PteFlags::X.bits()) != 0
    }

    pub fn is_writable_without_readable(self) -> bool {
        self.0 & PteFlags::W.bits() != 0 && self.0 & PteFlags::R.bits() == 0
    }
}

const PAGE_TABLE_ENTRIES: usize = 512;
const _: () = assert!(
    core::mem::size_of::<PageTableEntry>() * PAGE_TABLE_ENTRIES == 4096,
    "one page table page must hold exactly 512 page table entries"
);

const MAX_PAGE_TABLE_FRAMES: usize = 128;

fn pte_array(ppn: PhysPageNum) -> &'static mut [PageTableEntry; PAGE_TABLE_ENTRIES] {
    // Safety：ppn 来自页帧分配器且由 PageTable 持有，PageTable 存活期间一直有效。
    unsafe { &mut *(ppn.addr().0 as *mut [PageTableEntry; PAGE_TABLE_ENTRIES]) }
}

fn pte_at(ppn: PhysPageNum, index: usize) -> &'static mut PageTableEntry {
    // Safety：同 pte_array；index 来自 9 位页内索引，必然小于 512。
    &mut pte_array(ppn)[index]
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: [Option<FrameTracker>; MAX_PAGE_TABLE_FRAMES],
    frame_count: usize,
}

impl PageTable {
    pub fn new() -> Option<Self> {
        let root = frame_alloc()?;
        let root_ppn = root.ppn();
        let mut frames: [Option<FrameTracker>; MAX_PAGE_TABLE_FRAMES] =
            core::array::from_fn(|_| None);
        frames[0] = Some(root);
        Some(Self {
            root_ppn,
            frames,
            frame_count: 1,
        })
    }

    pub fn root_ppn(&self) -> PhysPageNum {
        self.root_ppn
    }

    fn alloc_frame(&mut self) -> Option<PhysPageNum> {
        if self.frame_count >= MAX_PAGE_TABLE_FRAMES {
            return None;
        }
        let frame = frame_alloc()?;
        let ppn = frame.ppn();
        self.frames[self.frame_count] = Some(frame);
        self.frame_count += 1;
        Some(ppn)
    }

    fn rollback(&mut self, created: &[(PhysPageNum, usize)], old_count: usize) {
        for (parent_ppn, index) in created.iter() {
            *pte_at(*parent_ppn, *index) = PageTableEntry::empty();
        }
        self.frame_count = old_count;
        for frame in self.frames[old_count..].iter_mut() {
            frame.take();
        }
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&'static mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut current = self.root_ppn;
        let old_count = self.frame_count;
        let mut created: [(PhysPageNum, usize); 2] = [(PhysPageNum(0), 0); 2];
        let mut created_len = 0usize;

        for level in 0..2 {
            let index = indexes[level];
            let entry = pte_at(current, index);
            if entry.is_valid() {
                if entry.is_leaf() {
                    self.rollback(&created[..created_len], old_count);
                    return None;
                }
                current = entry.ppn();
                continue;
            }

            let parent_ppn = current;
            let Some(child_ppn) = self.alloc_frame() else {
                self.rollback(&created[..created_len], old_count);
                return None;
            };
            created[created_len] = (parent_ppn, index);
            created_len += 1;
            *pte_at(parent_ppn, index) = PageTableEntry::new(child_ppn, PteFlags::V);
            current = child_ppn;
        }

        Some(pte_at(current, indexes[2]))
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&'static mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut current = self.root_ppn;

        for level in 0..2 {
            let entry = &pte_array(current)[indexes[level]];
            if !entry.is_valid() || entry.is_leaf() {
                return None;
            }
            current = entry.ppn();
        }

        Some(pte_at(current, indexes[2]))
    }

    pub fn map(
        &mut self,
        vpn: VirtPageNum,
        ppn: PhysPageNum,
        flags: PteFlags,
    ) -> Result<(), &'static str> {
        if flags.contains(PteFlags::W) && !flags.contains(PteFlags::R) {
            return Err("page-table map: W requires R");
        }
        if !flags.intersects(PteFlags::R | PteFlags::W | PteFlags::X) {
            return Err("page-table map: leaf needs R, W or X");
        }

        let entry = self
            .find_pte_create(vpn)
            .ok_or("page-table map: cannot create the page table entry")?;
        if entry.is_valid() {
            return Err("page-table map: virtual page is already mapped");
        }

        *entry = PageTableEntry::new(ppn, PteFlags::V | flags);
        Ok(())
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        let entry = self.find_pte(vpn)?;
        if entry.is_valid() && entry.is_leaf() {
            Some(*entry)
        } else {
            None
        }
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) -> Result<(), &'static str> {
        let entry = self
            .find_pte(vpn)
            .ok_or("page-table unmap: virtual page is not mapped")?;
        if !entry.is_valid() || !entry.is_leaf() {
            return Err("page-table unmap: virtual page is not mapped");
        }
        *entry = PageTableEntry::empty();
        Ok(())
    }
}

impl Drop for PageTable {
    fn drop(&mut self) {
        for frame in self.frames[..self.frame_count].iter_mut() {
            frame.take();
        }
    }
}
