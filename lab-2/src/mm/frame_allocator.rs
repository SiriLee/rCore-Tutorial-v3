use core::hint::spin_loop;
use core::ptr::write_bytes;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::{PhysAddr, PhysPageNum, PAGE_SIZE};

/// 空闲链表头保存的是物理地址，0 表示链表为空。
static HEAD: AtomicUsize = AtomicUsize::new(0);
static START_PPN: AtomicUsize = AtomicUsize::new(0);
static END_PPN: AtomicUsize = AtomicUsize::new(0);
static FRAME_LOCK: AtomicBool = AtomicBool::new(false);

#[inline]
fn lock() {
    while FRAME_LOCK.swap(true, Ordering::Acquire) {
        spin_loop();
    }
}

#[inline]
fn unlock() {
    FRAME_LOCK.store(false, Ordering::Release);
}

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
        let ppn = self.ppn.0;
        let start = START_PPN.load(Ordering::Relaxed);
        let end = END_PPN.load(Ordering::Relaxed);
        assert!(
            start <= ppn && ppn < end,
            "frame drop: ppn {:#x} out of managed range",
            ppn
        );

        let page_addr = ppn << 12;
        assert!(
            page_addr & (PAGE_SIZE - 1) == 0,
            "frame drop: unaligned page"
        );

        lock();
        let mut current = HEAD.load(Ordering::Relaxed);
        let mut steps = 0;
        while current != 0 {
            if current == page_addr {
                unlock();
                panic!("frame drop: page {:#x} is already free", page_addr);
            }
            assert!(
                (current >> 12) < end,
                "frame drop: free list left managed range"
            );
            unsafe {
                current = *(current as *const usize);
            }
            steps += 1;
            if steps > end - start {
                unlock();
                panic!("frame drop: free list is corrupted");
            }
        }
        unsafe {
            *(page_addr as *mut usize) = HEAD.load(Ordering::Relaxed);
        }
        HEAD.store(page_addr, Ordering::Relaxed);
        unlock();
    }
}

/// # Safety
/// `kernel_end` 之后到 `memory_end` 的地址必须是可写、页对齐后可分配的 RAM。
pub unsafe fn frame_allocator_init(kernel_end: PhysAddr, memory_end: PhysAddr) {
    let start_ppn = kernel_end.ceil().0;
    let end_ppn = memory_end.floor().0;
    assert!(
        start_ppn < end_ppn,
        "frame allocator: no allocatable physical page"
    );

    START_PPN.store(start_ppn, Ordering::Relaxed);
    END_PPN.store(end_ppn, Ordering::Relaxed);
    HEAD.store(0, Ordering::Relaxed);

    let mut head = 0;
    for ppn in start_ppn..end_ppn {
        let page_addr = ppn << 12;
        unsafe {
            *(page_addr as *mut usize) = head;
        }
        head = page_addr;
    }
    HEAD.store(head, Ordering::Relaxed);
}

pub fn frame_alloc() -> Option<FrameTracker> {
    lock();
    let head = HEAD.load(Ordering::Relaxed);
    if head == 0 {
        unlock();
        return None;
    }
    let next = unsafe { *(head as *const usize) };
    HEAD.store(next, Ordering::Relaxed);
    unlock();

    unsafe {
        write_bytes(head as *mut u8, 0, PAGE_SIZE);
    }
    Some(FrameTracker {
        ppn: PhysPageNum(head >> 12),
    })
}

pub fn free_frame_count() -> usize {
    lock();
    let start = START_PPN.load(Ordering::Relaxed);
    let end = END_PPN.load(Ordering::Relaxed);
    let mut current = HEAD.load(Ordering::Relaxed);
    let mut count = 0usize;
    let mut steps = 0usize;
    while current != 0 {
        if (current >> 12) >= end || steps > end.saturating_sub(start) {
            unlock();
            panic!("free_frame_count: free list is corrupted");
        }
        unsafe {
            current = *(current as *const usize);
        }
        count += 1;
        steps += 1;
    }
    unlock();
    count
}

pub fn is_managed(ppn: PhysPageNum) -> bool {
    let start = START_PPN.load(Ordering::Relaxed);
    let end = END_PPN.load(Ordering::Relaxed);
    start <= ppn.0 && ppn.0 < end
}
