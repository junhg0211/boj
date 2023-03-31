class Queue:
    def __init__(self):
        self.rear = 0
        self.front = 0
        self.values = dict()

    def enqueue(self, value):
        self.values[self.rear] = value
        self.rear += 1

    def dequeue(self):
        if self.is_empty():
            raise ValueError

        result = self.values.pop(self.front)
        self.front += 1
        return result

    def is_empty(self):
        return self.rear == self.front


def main():
    queue = Queue()

    for i in range(1, int(input()) + 1):
        queue.enqueue(i)

    i = 0
    while not queue.is_empty():
        if i % 2 == 0:
            print(queue.dequeue(), end=' ')
        else:
            queue.enqueue(queue.dequeue())

        i += 1

if __name__ == '__main__':
    main()
