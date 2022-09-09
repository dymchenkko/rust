#![feature(strict_provenance)]

use crate::alloc::{GlobalAlloc, Layout, System};
use unroll::unroll_for_loops;
use crate::cell::UnsafeCell;
use risc0_zkvm_platform::{memory, WORD_SIZE};

struct BumpPointerAlloc {
    head: UnsafeCell<usize>,
    end: usize,
}
// SAFETY: single threaded environment
unsafe impl Sync for BumpPointerAlloc {}

static mut HEAP: BumpPointerAlloc =
    BumpPointerAlloc { head: UnsafeCell::new(memory::HEAP.start()), end: memory::HEAP.end() };

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = HEAP.head.get();

        // move start up to the next alignment boundary
        let alloc_start = (*head + WORD_SIZE) & !(WORD_SIZE - 1);
        let alloc_end = alloc_start.checked_add(layout.size()).unwrap();
        if alloc_end > HEAP.end {
            panic!("out of heap");
        } else {
            *head = alloc_end;
            alloc_start as *mut u8
        }
    }

    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // this allocator never deallocates memory
    }

    #[stable(feature = "global_alloc", since = "1.28.0")]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        // SAFETY: the safety contract for `alloc` must be upheld by the caller.
        let ptr = unsafe { self.alloc(layout) };
        if !ptr.is_null() {
            // SAFETY: as allocation succeeded, the region from `ptr`
            // of size `size` is guaranteed to be valid for writes.
            unsafe { ptr::write_bytes(ptr, 0, size) };
        }
        ptr
    }
}

pub unsafe extern "C" fn memset(s: *mut u8, c: u8, n: usize) -> *mut u8 {
    if n % 128 == 0 && s.addr() % 4 == 0 && c == 0 {
        let mut bptr = s as * mut u32;
        let mut size = n;
        while (size != 0) {
            for i in 0..31 {
                bptr.offset(i as isize).write_volatile(0);
            }
            bptr = bptr.offset(32);
            size = size - 128;
        }
        bptr as * mut u8 
    }
    else {
        for _ in 0..n {
            s.offset(n as isize).write(c);
        }
        s
    }
}


