from sys import stdin
from collections import deque

input = stdin.readline


def main():
    n, m = map(int, input().split())
    queue = deque(range(1, n + 1))

    result = 0

    for index in map(int, input().split()):
        position = queue.index(index)
        backward_position = abs(-position % len(queue))

        if position < backward_position:
            for _ in range(position):
                queue.append(queue.popleft())
            result += position
        else:
            for _ in range(backward_position):
                queue.appendleft(queue.pop())
            result += backward_position

        queue.popleft()

    print(result)


if __name__ == '__main__':
    main()
