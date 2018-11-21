#!/usr/bin/python

class Queue:
    def __init__(self):
        self.queue_size = 10
        self.queue = []
    
    def enqueue(self, x):
        self.queue.insert(0, x)
    
    def dequeue(self):
        return self.queue.pop()

    def print_queue(self):
        print self.queue

the_queue = Queue()

the_queue.enqueue(1)
the_queue.enqueue(2)
the_queue.enqueue(3)
the_queue.print_queue()
y = the_queue.dequeue()
print(y)
the_queue.print_queue()
the_queue.enqueue(4)
the_queue.print_queue()


