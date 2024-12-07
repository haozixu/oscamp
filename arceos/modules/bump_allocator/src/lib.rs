#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
        }
    }
}

const fn align_down(x: usize, p2align: usize) -> usize {
    x & !(p2align - 1)
}

const fn align_up(x: usize, p2align: usize) -> usize {
    (x + p2align - 1) & !(p2align - 1)
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = align_down(self.end, PAGE_SIZE);
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> allocator::AllocResult {
        todo!() // unsupported
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {        
        let pos = align_up(self.b_pos, layout.align());
        let new_b_pos = pos + layout.size();
        if new_b_pos > self.p_pos {
            return Err(allocator::AllocError::NoMemory);
        }

        self.b_pos = new_b_pos;
        unsafe {
            Ok(core::ptr::NonNull::new_unchecked(pos as *mut u8))
        }
    }

    fn dealloc(&mut self, _pos: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
        // do nothing
    }

    fn total_bytes(&self) -> usize {
        todo!()
    }

    fn used_bytes(&self) -> usize {
        todo!()
    }

    fn available_bytes(&self) -> usize {
        todo!()
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(
        &mut self,
        _num_pages: usize,
        _align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        todo!()
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        todo!()
    }

    fn available_pages(&self) -> usize {
        todo!()
    }
}
