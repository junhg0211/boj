from collections import deque
from sys import stdin

input = stdin.readline


def main():
    left = deque(input().rstrip())
    right = deque()

    for _ in range(int(input())):
        command = input()

        if command[0] == 'L':
            if len(left) > 0:
                right.appendleft(left.pop())
            continue

        if command[0] == 'D':
            if len(right) > 0:
                left.append(right.popleft())
            continue

        if command[0] == 'B':
            if len(left) > 0:
                left.pop()
            continue

        if command[0] == 'P':
            left.append(command[2])

    left = ''.join(left)
    right = ''.join(right)
    print(left, right, sep='')


if __name__ == '__main__':
    main()
