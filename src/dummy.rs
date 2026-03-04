use crate::Allocator;
use core::ptr;

pub struct System {
    _priv: (),
}

impl System {
    /// Creates a new instance.
    pub const fn new() -> System {
        System { _priv: () }
    }
}

unsafe impl Allocator for System {
    /// Implements alloc.
    fn alloc(&self, _size: usize) -> (*mut u8, usize, u32) {
        (ptr::null_mut(), 0, 0)
    }

    /// Implements remap.
    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        ptr::null_mut()
    }

    /// Implements free part.
    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    /// Implements free.
    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        false
    }

    /// Implements can release part.
    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    /// Implements allocates zeros.
    fn allocates_zeros(&self) -> bool {
        false
    }

    /// Implements page size.
    fn page_size(&self) -> usize {
        1
    }
}
