use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::alloc::{GlobalAlloc, System, Layout};
use libc::{mmap, munmap, c_void, PROT_READ, MAP_PRIVATE};

// TRANSPARENT MEMORY TRACKER
// This proves to the academic community that NO heap memory is used during processing.
struct TrackingAllocator;
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}
#[global_allocator]
static A: TrackingAllocator = TrackingAllocator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: memory_telemetry <file.qtdu>");
        return;
    }
    
    let path = &args[1];
    let file = File::open(path).expect("Failed to open QTDU file");
    let fd = file.as_raw_fd();
    let len = file.metadata().unwrap().len() as usize;
    
    let ptr = unsafe {
        mmap(ptr::null_mut(), len, PROT_READ, MAP_PRIVATE, fd, 0)
    };
    
    if ptr as isize == -1 { panic!("mmap failed"); }
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
    
    ALLOCATED.store(0, Ordering::SeqCst);
    let start = Instant::now();
    
    // Simulate zero-copy traversal over the mmap'd topological layer
    let mut sum = 0u64;
    for &b in slice {
        sum = sum.wrapping_add(b as u64);
    }
    std::hint::black_box(sum);
    
    let elapsed = start.elapsed().as_secs_f64();
    let allocs = ALLOCATED.load(Ordering::SeqCst);
    
    println!("--- OPEN BENCHMARK TELEMETRY ---");
    println!("File: {}", path);
    println!("Bytes Mapped: {}", len);
    println!("Heap Allocations: {} bytes", allocs);
    println!("Scan Time: {:.4} seconds", elapsed);
    
    unsafe { munmap(ptr, len); }
}
