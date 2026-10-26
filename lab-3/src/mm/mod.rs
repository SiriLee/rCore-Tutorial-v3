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

fn pass_text(passed: bool) -> &'static str {
    if passed {
        "PASS"
    } else {
        "FAIL"
    }
}

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

fn page_table_test() -> bool {
    let before = free_frame_count();

    let mut page_table = match PageTable::new() {
        Some(page_table) => page_table,
        None => {
            println!("page-table test: cannot create the page table");
            return false;
        }
    };
    let target = match frame_alloc() {
        Some(frame) => frame,
        None => {
            println!("page-table test: cannot allocate the target frame");
            return false;
        }
    };
    let other = match frame_alloc() {
        Some(frame) => frame,
        None => {
            println!("page-table test: cannot allocate the spare frame");
            return false;
        }
    };

    let vpn = VirtPageNum(0x12345);
    if page_table.translate(vpn).is_some() {
        println!("page-table test: unexpected mapping before map");
        return false;
    }

    if page_table
        .map(vpn, target.ppn(), PteFlags::R | PteFlags::W)
        .is_err()
    {
        println!("page-table test: map failed");
        return false;
    }
    let mapped = match page_table.translate(vpn) {
        Some(entry) => entry,
        None => {
            println!("page-table test: translated entry is missing");
            return false;
        }
    };
    if mapped.ppn() != target.ppn() {
        println!("page-table test: translated ppn does not match");
        return false;
    }

    if page_table.map(vpn, other.ppn(), PteFlags::R).is_ok() {
        println!("page-table test: duplicate map was accepted");
        return false;
    }
    if page_table.translate(vpn) != Some(mapped) {
        println!("page-table test: duplicate map changed the original entry");
        return false;
    }

    if page_table.map(vpn, target.ppn(), PteFlags::W).is_ok() {
        println!("page-table test: W-only mapping was accepted");
        return false;
    }
    if page_table.map(vpn, target.ppn(), PteFlags(0)).is_ok() {
        println!("page-table test: mapping without leaf permission was accepted");
        return false;
    }

    if page_table.unmap(vpn).is_err() {
        println!("page-table test: unmap failed");
        return false;
    }
    if page_table.translate(vpn).is_some() {
        println!("page-table test: entry is still mapped after unmap");
        return false;
    }
    if page_table.unmap(vpn).is_ok() {
        println!("page-table test: second unmap was accepted");
        return false;
    }

    // 让测试页表、目标 frame 和临时 frame 全部离开作用域。
    drop(page_table);
    drop(target);
    drop(other);

    if free_frame_count() != before {
        println!("page-table test: page frames were leaked");
        return false;
    }
    true
}

pub fn run_tests() -> bool {
    let address_ok = address_test();
    println!("lab-2 address test: {}", pass_text(address_ok));

    let frame_ok = frame_test();
    println!("lab-2 frame test: {}", pass_text(frame_ok));

    let page_table_ok = page_table_test();
    println!("lab-2 page-table test: {}", pass_text(page_table_ok));

    let prerequisites_ok = address_ok && frame_ok && page_table_ok;
    let kernel_ok = prerequisites_ok && kernel_space_init().is_ok();
    println!("lab-2 kernel-space test: {}", pass_text(kernel_ok));

    prerequisites_ok && kernel_ok
}
