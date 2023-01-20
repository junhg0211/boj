DIVIDER = 1_000_000_007
DDIVIDER = DIVIDER * 2


def power(a, b: int):
    i = a
    result = 1
    while b:
        if b & 1:
            result *= i
            result %= DDIVIDER
        b >>= 1
        i *= i
        i %= DDIVIDER
    return result


def main():
    n = int(input())

    a = power((1 + 5**.5) / 2, n) - power((1 - 5**.5) / 2, n)
    print(round(a / 5**.5) % DIVIDER)


if __name__ == '__main__':
    main()
