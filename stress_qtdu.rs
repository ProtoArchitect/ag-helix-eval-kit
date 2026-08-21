use std::time::{Instant, Duration};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// Custom Allocator to prove Zero-Copy
use std::alloc::{GlobalAlloc, System, Layout};

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
    println!("Init: allocating 1GB dummy payload...");
    let size = 1_000_000_000; // 1 GB
    let mut data = vec![0u8; size];
    
    // Pretend this is our Qtdu Layer 1 spine
    // We add some mutations to prevent total optimization bypass
    for i in (0..size).step_by(3000) { data[i] = 1; }

    println!("Payload created. Resetting memory tracker to 0...");
    ALLOCATED.store(0, Ordering::SeqCst);
    
    let thread_count = 12; // Will spawn multiple threads to heat up the CPU
    println!("Starting 15-second multi-core SWAR stress test on {} threads...", thread_count);
    
    let start = Instant::now();
    let mut handles = vec![];
    
    for _ in 0..thread_count {
        // We use raw pointer sharing to avoid Arc allocations
        let ptr = data.as_ptr() as usize;
        let len = data.len();
        
        handles.push(thread::spawn(move || {
            let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
            let mut local_scanned = 0u64;
            let mut cycles = 0u64;
            let t_start = Instant::now();
            
            while t_start.elapsed() < Duration::from_secs(15) {
                let mut idx = 0;
                let mut matches = 0;
                while idx + 8 <= slice.len() {
                    let chunk = u64::from_le_bytes(slice[idx..idx+8].try_into().unwrap());
                    if chunk == 0 { matches += 1; }
                    idx += 8;
                }
                std::hint::black_box(matches);
                local_scanned += slice.len() as u64;
                cycles += 1;
            }
            (local_scanned, cycles)
        }));
    }
    
    let mut total_scanned = 0u64;
    for h in handles {
        let (scanned, _) = h.join().unwrap();
        total_scanned += scanned;
    }
    
    let elapsed = start.elapsed().as_secs_f64();
    let allocs_during_test = ALLOCATED.load(Ordering::SeqCst);
    
    let gb_scanned = total_scanned as f64 / 1_000_000_000.0;
    
    println!("--- TEST COMPLETE ---");
    println!("SCANNED_GB={:.2}", gb_scanned);
    println!("ELAPSED_SEC={:.2}", elapsed);
    println!("THROUGHPUT_GB_S={:.2}", gb_scanned / elapsed);
    println!("ALLOCATIONS={}", allocs_during_test);
}
