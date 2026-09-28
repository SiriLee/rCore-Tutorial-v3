pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VirtPageNum(pub usize);

impl PhysAddr {
    pub fn floor(self) -> PhysPageNum {
        PhysPageNum(self.0 >> PAGE_SIZE_BITS)
    }

    pub fn ceil(self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) >> PAGE_SIZE_BITS)
    }

    pub fn page_offset(self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}

impl VirtAddr {
    pub fn floor(self) -> VirtPageNum {
        VirtPageNum(self.0 >> PAGE_SIZE_BITS)
    }

    pub fn ceil(self) -> VirtPageNum {
        VirtPageNum((self.0 + PAGE_SIZE - 1) >> PAGE_SIZE_BITS)
    }

    pub fn page_offset(self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}

impl PhysPageNum {
    pub fn addr(self) -> PhysAddr {
        PhysAddr(self.0 << PAGE_SIZE_BITS)
    }
}

impl VirtPageNum {
    pub fn addr(self) -> VirtAddr {
        VirtAddr(self.0 << PAGE_SIZE_BITS)
    }

    pub fn indexes(self) -> [usize; 3] {
        let mut value = self.0;
        let mut indexes = [0usize; 3];
        for index in (0..3).rev() {
            indexes[index] = value & 0x1ff;
            value >>= 9;
        }
        indexes
    }
}
