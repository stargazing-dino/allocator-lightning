#![feature(allocator_api)]
use std::alloc::{AllocError, Allocator, Layout};
use std::cell::{Cell, UnsafeCell};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

pub struct MyArena<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    next: Cell<usize>,
}

impl<const N: usize> MyArena<N> {
    pub const fn new() -> Self {
        Self {
            buf: UnsafeCell::new([MaybeUninit::uninit(); N]),
            next: Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Allocator for MyArena<N> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if layout.size() == 0 {
            let ptr = unsafe { NonNull::new_unchecked(layout.align() as *mut u8) };

            return Ok(NonNull::slice_from_raw_parts(ptr, 0));
        }

        let base = self.buf.get() as *mut u8 as usize;

        todo!()
    }
}
