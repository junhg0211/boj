from sys import stdin
from collections import deque


def main():
    instruction_count = int(input())

    numbers = deque()

    for _ in range(instruction_count):
        tokens = stdin.readline().rstrip().split()

        if tokens[0] == '1':
            numbers.appendleft(tokens[1])
            continue

        if tokens[0] == '2':
            numbers.append(tokens[1])
            continue

        if tokens[0] == '3':
            if numbers:
                print(numbers.popleft())
            else:
                print('-1')
            continue

        if tokens[0] == '4':
            if numbers:
                print(numbers.pop())
            else:
                print('-1')
            continue

        if tokens[0] == '5':
            print(len(numbers))
            continue

        if tokens[0] == '6':
            print(int(len(numbers) == 0))
            continue

        if tokens[0] == '7':
            if numbers:
                print(numbers[0])
            else:
                print('-1')
            continue

        # if tokens[0] == '8'
        if numbers:
            print(numbers[-1])
        else:
            print('-1')


if __name__ == '__main__':
    main()
