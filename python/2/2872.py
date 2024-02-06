from sys import stdin

input = stdin.readline


def main():
    count = int(input())
    numbers = [int(input()) for _ in range(count)]

    result = 0
    must_get = count
    for number in reversed(numbers):
        if must_get != number:
            result += 1
        else:
            must_get -= 1

    print(result)


if __name__ == '__main__':
    main()
