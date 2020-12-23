/*
 * Same C library source with a function to call
 */
#include <stdio.h>
#include "dafunc.h"

int print_something() {
	printf("I'm in the library\n");
	return 0;
}

