/*
Simple C Style Logger
Khalid Akash 2018 - MIT License
*/

#ifndef __LOGGER__
#define __LOGGER__
#include <stdio.h>
#define DEBUG

/* FOREGROUND */
#ifndef WIN32
#define RST "\x1B[0m"
#define KRED "\x1B[31m"
#define KGRN "\x1B[32m"
#define KYEL "\x1B[33m"
#define KBLU "\x1B[34m"
#define KMAG "\x1B[35m"
#define KCYN "\x1B[36m"
#define KWHT "\x1B[37m"
#define BOLD(x) "\x1B[1m" x RST
#define UNDL(x) "\x1B[4m" x RST
#else
#define RST ""
#define KRED ""
#define KGRN ""
#define KYEL ""
#define KBLU ""
#define KMAG ""
#define KCYN ""
#define KWHT ""
#define BOLD(x) "" x RST
#define UNDL(x) "" x RST
#endif

#define RED(x) KRED x RST
#define GRN(x) KGRN x RST
#define YEL(x) KYEL x RST
#define BLU(x) KBLU x RST
#define MAG(x) KMAG x RST
#define CYN(x) KCYN x RST
#define WHT(x) KWHT x RST

#ifndef NDEBUG
#define debug_log(fmt, ...)                                \
    do                                                     \
    {                                                      \
        fprintf(stderr, YEL("DEBUG %s - %d:%s(): " fmt), __TIME__, \
                __LINE__, __func__, __VA_ARGS__);          \
    } while (0)
#define debug_logln(fmt, ...)                                   \
    do                                                          \
    {                                                           \
        fprintf(stderr, YEL("DEBUG %s - %d:%s(): " fmt "\n"), __TIME__, \
                __LINE__, __func__, __VA_ARGS__);               \
    } while (0)
#else
#define debug_logln(...)
#define debug_log(...)
#endif

#ifdef WIN32
/*Stops console from immediately disappearing in MSVS*/
inline void pause_terminal()
{
	char pause_buffer[256];
	scanf(pause_buffer);
}
#else
inline void pause_terminal() {};
#endif

#define err_log(fmt, ...)                                         \
    do                                                            \
    {                                                             \
        fprintf(stderr, BOLD(RED("ERROR %s - %d:%s(): " fmt)), __TIME__, \
                __LINE__, __func__, __VA_ARGS__);                 \
		pause_terminal(); \
    } while (0)

#define err_logln(fmt, ...)                                            \
    do                                                                 \
    {                                                                  \
        fprintf(stderr, BOLD(RED("ERROR %s - %d:%s(): " fmt "\n")), __TIME__, \
                __LINE__, __func__, __VA_ARGS__);                      \
		pause_terminal(); \
    } while (0)
#endif