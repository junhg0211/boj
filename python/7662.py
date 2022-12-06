from collections import deque
from sys import stdin

input = stdin.readline


def get_index(queue, number):
    start = 0
    end = len(queue)

    while end != start:
        anchor = (start + end) // 2
        middle = queue[anchor]

        if number > middle:
            start = anchor+1
        elif number < middle:
            end = anchor
        else:
            return anchor

    return end


def tick():
    queue = deque()
    for _ in range(int(input())):
        command, args = input().split()

        if command == 'D':
            if not queue:
                continue

            if args == '-1':
                queue.popleft()
                continue

            queue.pop()

        elif command == 'I':
            args = int(args)
            index = get_index(queue, args)
            queue.insert(index, args)

    if queue:
        print(queue[-1], queue[0])
    else:
        print('EMPTY')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
