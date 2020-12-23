//
// Minimal C++ source file with object to show how to call a C function from
// libdafunc.a which is statically linked in by g++
//
#include <iostream>
#include "dafunc.h"

using namespace std;

class A {
public:
	void call_dafunc();
};

void A::call_dafunc()
{
	cout << "Calling libdafunc" << endl;

	// This file is pure C function linked in
	print_something();
}

int main()
{
	A a_int;

	a_int.call_dafunc();
	return 0;
}

