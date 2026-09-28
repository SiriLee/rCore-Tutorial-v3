mod address;
mod frame_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum, PAGE_SIZE};
pub use frame_allocator::{frame_alloc, frame_allocator_init, FrameTracker};
pub use memory_set::{kernel_space_init, kernel_token};
pub use page_table::{PageTable, PageTableEntry, PteFlags};

fn address_test() -> bool {
    let va = VirtAddr(0x12345);
    va.floor().0 == 0x12
        && va.ceil().0 == 0x13
        && va.page_offset() == 0x345
        && VirtPageNum(0x12345).indexes() == [0x0, 0x91, 0x145]
}

/// TODO(task 7): 依次运行地址、页帧、页表和内核地址空间测试。
pub fn run_tests() -> bool {
    address_test()
}
