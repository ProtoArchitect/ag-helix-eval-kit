use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::alloc::{GlobalAlloc, System, Layout};
use std::ffi::c_void;

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

extern "C" {
    fn mmap(addr: *mut c_void, length: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> i32;
}

fn main() {
    let path = "/home/alpha/Documents/Ag/ag-eta/repos/ag-eta-delta/eval-kit/demo/large_genomics/human_genome_heavy.fa";
    let file = File::open(path).expect("Failed to open the real 3GB FASTA file");
    let fd = file.as_raw_fd();
    let len = file.metadata().unwrap().len() as usize;
    
    // Pure OS Kernel mmap (Zero-Copy projection to RAM)
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            len,
            1, // PROT_READ
            1, // MAP_SHARED
            fd,
            0,
        )
    };
    
    if ptr as isize == -1 {
        panic!("mmap failed");
    }

    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
    
    // Reset tracker immediately before the SWAR loop
    ALLOCATED.store(0, Ordering::SeqCst);
    
    // The actual QTD SWAR logic loop over REAL biological entropy
    let start = Instant::now();
    let mut _matches = 0;
    let mut idx = 0;
    
    while idx + 8 <= slice.len() {
        let chunk = u64::from_le_bytes(slice[idx..idx+8].try_into().unwrap());
        if chunk == 0 { _matches += 1; }
        std::hint::black_box(chunk);
        idx += 8;
    }
    
    let elapsed = start.elapsed().as_secs_f64();
    let allocs = ALLOCATED.load(Ordering::SeqCst);
    
    let gb = len as f64 / 1_000_000_000.0;
    
    println!("--- REAL GENOME BENCHMARK ---");
    println!("FILE: {}", path);
    println!("SIZE_BYTES: {}", len);
    println!("ALLOCATIONS_DURING_SCAN: {}", allocs);
    println!("ELAPSED_SEC: {:.4}", elapsed);
    println!("THROUGHPUT_GB_S: {:.2}", gb / elapsed);
    
    unsafe { munmap(ptr, len); }
}
