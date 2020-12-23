all: clean
	gcc -c dafunc.c
	ar rcs libdafunc.a dafunc.o
	g++ tryme.cpp -o tryme libdafunc.a

clean:
	rm -f *.o *.a tryme
