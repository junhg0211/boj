from sys import stdin
from math import sqrt, ceil

input = stdin.readline


def tick():
    number = int(input())

    for i in range(2, ceil(sqrt(number) + 1)):
        count = 0
        while number % i == 0:
            number //= i
            count += 1

        if count > 0:
            print(i, count)

    if number != 1:
        print(number, 1)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
