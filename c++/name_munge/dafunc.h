/*
 * Sampe library header file that can be included in both C and C++ source
 * files.
 */
#ifndef _DAFUNC_H_
#define _DAFUNC_H_

/* __cplusplus is defined only if we are using a C++ compiler such as g++ */
#ifdef __cplusplus
/*
 * extern "C" tells the C++ compiler to not munge the name of this function
 * so that we will not get hard to trace linker errors telling us the symbol
 * is not found.
 */
extern "C" {
#endif
	int print_something();
#ifdef __cplusplus
}
#endif

#endif
