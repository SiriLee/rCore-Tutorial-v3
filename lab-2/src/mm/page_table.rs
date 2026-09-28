use super::{frame_alloc, FrameTracker, PhysPageNum, VirtPageNum};

#[derive(Copy, Clone)]
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
}

impl core::ops::BitOr for PteFlags {
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
}

const MAX_PAGE_TABLE_FRAMES: usize = 128;

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: [Option<FrameTracker>; MAX_PAGE_TABLE_FRAMES],
    frame_count: usize,
}

impl PageTable {
    pub fn new() -> Option<Self> {
        let root = frame_alloc()?;
        let root_ppn = root.ppn();
        let mut frames = core::array::from_fn(|_| None);
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

    pub fn map(
        &mut self,
        vpn: VirtPageNum,
        ppn: PhysPageNum,
        flags: PteFlags,
    ) -> Result<(), &'static str> {
        let _ = (vpn, ppn, flags, &self.frames, self.frame_count);
        // TODO(task 5): 建立缺失的中间页表，并拒绝重复映射与非法 W-only 权限。
        Err("page-table map is not implemented")
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) -> Result<(), &'static str> {
        let _ = vpn;
        // TODO(task 5): 清除已存在的叶子 PTE；未映射时返回错误。
        Err("page-table unmap is not implemented")
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        let _ = vpn;
        // TODO(task 5): 只查询，不在查询路径分配页表。
        None
    }
}
