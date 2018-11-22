#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn get_rdtsc() -> u64 {
    unsafe {
        let rdtsc_val: u64;
        asm!("mfence");
        asm!(
            "rdtsc"
            :"=r"(rdtsc_val)
        );
        asm!("mfence");
        rdtsc_val << 32 | 0
    }
}

pub fn mem_access(addr: *const usize) {
    unsafe {
        asm!("movq (%0), %%rax\n"
        :
        : "c" (addr)
        : "rax"
        );
    }
}

pub fn flush_cache_line(addr: *const usize) {
    unsafe {
        asm!("clflush 0(%0)\n"
        :  
        : "c" (addr)
        : "rax"
        );
    }
}