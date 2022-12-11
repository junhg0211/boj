from math import log2


def main():
    _, a, b = map(int, input().split())
    a, b = a - 1, b - 1

    print(int(log2(a^b)) + 1)


if __name__ == '__main__':
    main()
