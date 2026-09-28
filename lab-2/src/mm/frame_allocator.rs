use super::{PhysAddr, PhysPageNum, PAGE_SIZE};

pub struct FrameTracker {
    ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn ppn(&self) -> PhysPageNum {
        self.ppn
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        // TODO(task 4): 把物理页归还空闲链表，并检查重复或越界释放。
    }
}

/// # Safety
/// `kernel_end` 之后到 `memory_end` 的地址必须是可写、页对齐后可分配的 RAM。
pub unsafe fn frame_allocator_init(kernel_end: PhysAddr, memory_end: PhysAddr) {
    let _ = (kernel_end, memory_end, PAGE_SIZE);
    // TODO(task 4): 建立空闲物理页链表并清零分配状态。
}

pub fn frame_alloc() -> Option<FrameTracker> {
    // TODO(task 4): 从空闲链表移除一页、清零并返回 FrameTracker。
    None
}
