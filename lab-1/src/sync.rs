use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::arch;

const MAX_HARTS: usize = 8;

struct PerHart<T> {
    cells: [UnsafeCell<T>; MAX_HARTS],
}

unsafe impl<T> Sync for PerHart<T> {}

impl<T> PerHart<T> {
    #[inline]
    fn index(&self, hart: usize) -> *mut T {
        assert!(hart < MAX_HARTS, "hart id out of range");
        self.cells[hart].get()
    }
}

static INTR_NESTING: PerHart<usize> = PerHart {
    cells: [const { UnsafeCell::new(0) }; MAX_HARTS],
};
static INTR_PREV: PerHart<bool> = PerHart {
    cells: [const { UnsafeCell::new(false) }; MAX_HARTS],
};

#[inline]
fn intr_get() -> bool {
    arch::interrupts_enabled()
}

fn push_off() {
    let enabled = intr_get();
    arch::disable_interrupts();
    let hart = arch::hart_id();
    unsafe {
        let depth = &mut *INTR_NESTING.index(hart);
        if *depth == 0 {
            *INTR_PREV.index(hart) = enabled;
        }
        *depth += 1;
    }
}

fn pop_off() {
    let hart = arch::hart_id();
    unsafe {
        let depth = &mut *INTR_NESTING.index(hart);
        if *depth == 0 {
            panic!("pop_off: nesting depth underflow");
        }
        *depth -= 1;
        if *depth == 0 && *INTR_PREV.index(hart) {
            arch::enable_interrupts();
        }
    }
}

pub struct SpinLock {
    locked: AtomicBool,
    owner: AtomicUsize,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            owner: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_> {
        push_off();
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        self.owner.store(arch::hart_id(), Ordering::Relaxed);
        SpinLockGuard { lock: self }
    }

    fn unlock(&self) {
        self.owner.store(usize::MAX, Ordering::Relaxed);
        self.locked.store(false, Ordering::Release);
    }
}

unsafe impl Sync for SpinLock {}

pub struct SpinLockGuard<'a> {
    lock: &'a SpinLock,
}

impl Drop for SpinLockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock();
        pop_off();
    }
}

pub fn interrupt_nesting_selftest() -> bool {
    let original = intr_get();

    let outer = crate::sync::SpinLock::new();
    let inner = crate::sync::SpinLock::new();

    {
        let _outer_guard = outer.lock();
        if intr_get() {
            return false;
        }
        {
            let _inner_guard = inner.lock();
            if intr_get() {
                return false;
            }
        }
        if intr_get() {
            return false;
        }
    }

    intr_get() == original
}
