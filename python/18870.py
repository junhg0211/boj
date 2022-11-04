from sys import stdin


def main():
    input()

    numbers = tuple(map(int, stdin.readline().split()))
    sorteds = sorted(set(numbers))

    for number in numbers:
        print(sorteds.index(number), end=' ')


if __name__ == '__main__':
    main()
