from math import ceil


def main():
    a, b = map(int, input().split())

    print(ceil(a * b / 4840 / 5))


if __name__ == "__main__":
    main()
