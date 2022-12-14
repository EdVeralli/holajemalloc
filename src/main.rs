extern crate jemalloc_sys;
extern crate jemallocator;

// Work around https://github.com/gnzlbg/jemallocator/issues/19
#[global_allocator]
static A: jemallocator::Jemalloc = jemallocator::Jemalloc;

// #[test]
fn main() {
    unsafe {
        let ptr = jemalloc_sys::malloc(4); // Allocates size bytes of uninitialized memory.
        *(ptr as *mut u32) = 0xDECADE;
        assert_eq!(*(ptr as *mut u32), 0xDECADE);
        jemalloc_sys::free(ptr);
    }
}
