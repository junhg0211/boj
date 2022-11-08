from math import sqrt


def a(n):
    return int((-1 + sqrt(1 + 8*(n-1))) / 2) + 1


def main():
    from_number, to_number = map(int, input().split())
    result = 0
    for i in range(from_number, to_number + 1):
        result += a(i)

    print(result)


if __name__ == '__main__':
    main()
