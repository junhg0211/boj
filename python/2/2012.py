from collections import deque
from sys import stdin

input = stdin.readline


def main():
    count = int(input())

    numbers = list()
    nos = set(range(1, count + 1))
    for _ in range(count):
        number = int(input())
        numbers.append(number)
        if number in nos:
            nos.remove(number)
    numbers.sort()
    nos = deque(sorted(nos))

    result = 0
    for i in range(count):
        now = numbers[i]

        result += abs(i + 1 - now)

    print(result)


if __name__ == "__main__":
    main()
