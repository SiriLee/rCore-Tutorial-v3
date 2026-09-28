mod address;
mod frame_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum, PAGE_SIZE};
pub use frame_allocator::{frame_alloc, frame_allocator_init, FrameTracker};
pub use memory_set::{kernel_space_init, kernel_token};
pub use page_table::{PageTable, PageTableEntry, PteFlags};

/// TODO(task 7): 依次运行地址、页帧、页表和内核地址空间测试。
pub fn run_tests() -> bool {
    false
}
