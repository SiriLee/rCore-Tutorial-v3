mod address;
mod frame_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum, PAGE_SIZE};
pub use frame_allocator::{
    frame_alloc, frame_allocator_init, free_frame_count, is_managed, FrameTracker,
};
pub use memory_set::{kernel_space_init, kernel_token};
pub use page_table::{PageTable, PageTableEntry, PteFlags};

fn address_test() -> bool {
    let va = VirtAddr(0x12345);
    va.floor().0 == 0x12
        && va.ceil().0 == 0x13
        && va.page_offset() == 0x345
        && VirtPageNum(0x12345).indexes() == [0x0, 0x91, 0x145]
}

fn frame_test() -> bool {
    let before = free_frame_count();
    if before < 3 {
        println!("frame test: not enough free frames: {}", before);
        return false;
    }

    let a = match frame_alloc() {
        Some(frame) => frame,
        None => {
            println!("frame test: first allocation failed");
            return false;
        }
    };
    let b = match frame_alloc() {
        Some(frame) => frame,
        None => {
            println!("frame test: second allocation failed");
            return false;
        }
    };
    if a.ppn() == b.ppn() || !is_managed(a.ppn()) || !is_managed(b.ppn()) {
        println!("frame test: invalid frame pair");
        return false;
    }
    if free_frame_count() != before - 2 {
        println!("frame test: free count did not drop by two");
        return false;
    }

    let a_ppn = a.ppn();
    unsafe {
        (a_ppn.addr().0 as *mut u8).write_volatile(0x5a);
    }
    drop(a);

    let c = match frame_alloc() {
        Some(frame) => frame,
        None => {
            println!("frame test: reallocation after drop failed");
            return false;
        }
    };
    if c.ppn() != a_ppn {
        println!("frame test: freed frame was not reused");
        return false;
    }
    let first_byte = unsafe { (c.ppn().addr().0 as *const u8).read_volatile() };
    if first_byte != 0 {
        println!(
            "frame test: reused frame was not cleared: {:#x}",
            first_byte
        );
        return false;
    }

    drop(b);
    drop(c);

    if free_frame_count() != before {
        println!("frame test: free frame count was not restored");
        return false;
    }
    true
}

/// TODO(task 7): 依次运行地址、页帧、页表和内核地址空间测试。
pub fn run_tests() -> bool {
    address_test() && frame_test()
}
