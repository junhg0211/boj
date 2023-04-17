from sys import stdin
from math import inf

input = stdin.readline


def main():
    result = 0

    min_ = inf
    for _ in range(4):
        number = int(input())
        min_ = min(min_, number)
        result += number
    result -= min_

    min_ = inf
    for _ in range(2):
        number = int(input())
        min_ = min(min_, number)
        result += number
    result -= min_

    print(result)


if __name__ == '__main__':
    main()
