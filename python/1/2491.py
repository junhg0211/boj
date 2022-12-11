from sys import stdin

input = stdin.readline


def main():
    input()
    numbers = map(int, input().split())
    ascending_strike = 0
    descending_strike = 0
    max_strike = 0
    previous = None
    for number in numbers:
        if previous is None:
            ascending_strike = 1
            descending_strike = 1
            max_strike = 1
            previous = number
            continue

        if previous >= number:
            descending_strike += 1
        else:
            descending_strike = 1
        if previous <= number:
            ascending_strike += 1
        else:
            ascending_strike = 1

        max_strike = max(ascending_strike, descending_strike, max_strike)

        previous = number

    print(max_strike)


if __name__ == '__main__':
    main()
