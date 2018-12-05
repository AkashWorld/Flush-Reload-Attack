#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]

/*Gets processor time*/
#[inline]
pub fn get_rdtsc() -> u64 {
    unsafe {
        let rdtsc_val: u64;
        asm!("mfence \n
              rdtsc  \n
              mfence \n"
            :"=A"(rdtsc_val)
        );
        rdtsc_val
    }
}

/*Accesses location at addresss 'addr'*/
#[inline]
pub fn mem_access(addr: *const u8) {
    unsafe {
        asm!(
        "movq ($0), %rax\n"
        :
        : "r"(addr)
        : "rax"
        );
    }
}

/*Flushed the cache line that contains address 'addr'*/
#[inline]
pub fn flush_cache_line(addr: *const u8) {
    unsafe {
        asm!("clflush ($0)\n"
        :  
        : "r" (addr)
        : "rax"
        );
    }
}

/*AT&T Syntax Test*/
#[inline]
pub fn ret_mem(addr: *const u8) -> u64 {
    let mut val: u64 = 1000;
    unsafe {
        asm!(
        "movq ($0), $0\n"
        : "=r"(val)
        : "r"(addr)
        : "rax", "rbx"
        );
    }
    val
}

/*Returns time after a memory access at address 'addr'*/
#[inline]
pub fn full_reload_time(addr: *const u8) -> u64 {
    let time: u64;
    unsafe {
        asm!("mfence \n
        lfence \n
        rdtsc \n
        lfence \n
        movl %eax, %esi \n
        movl ($1), %eax \n
        lfence \n
        rdtsc \n
        subl %esi, %eax \n"
        :"=A" (time)
        :"r" (addr)
        :"%esi", "%eax"
        );
    }
    time
}

/*Returns time after a memory access and cache line flush, both at address 'addr'*/
#[inline]
pub fn full_flush_reload_time(addr: *const u8) -> u64 {
    let time: u64;
    unsafe {
        asm!("mfence \n
        lfence \n
        rdtsc \n
        lfence \n
        movl %eax, %esi \n
        movl ($1), %eax \n
        lfence \n
        rdtsc \n
        subl %esi, %eax \n
        clflush ($1) \n"
        :"=A" (time)
        :"r" (addr)
        :"%esi", "%eax"
        );
    }
    time
}