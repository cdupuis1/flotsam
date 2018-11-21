#!/usr/bin/python

# Use an empty list as the base of the stack
stack = []

def push(x):
    stack.append(x)

def pop():
    if not stack:
        print "List is empty"
        return

    tmp_val = stack[len(stack) - 1]
    stack.pop()
    return tmp_val

y = pop()
push(5)
push(4)
y = pop()
print(y)
y = pop()
print(y)


