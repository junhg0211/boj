from math import log10, floor


def main():
    n = int(input())

    result = 0
    while n:
        length = floor(log10(n))
        target = 10**length - 1
        count = n - target
        result += count * (length+1)
        result %= 1234567
        n -= count

    print(result)


if __name__ == '__main__':
    main()
