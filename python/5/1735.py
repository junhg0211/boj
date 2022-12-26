def gcd(a, b):
    if a < b:
        return gcd(b, a)

    while True:
        c = a % b
        if c == 0:
            return b
        a = b
        b = c


def main():
    a, b = map(int, input().split())
    c, d = map(int, input().split())

    numerator = a*d + c*b
    denominator = b*d

    divisor = gcd(numerator, denominator)
    numerator //= divisor
    denominator //= divisor

    print(numerator, denominator)


if __name__ == '__main__':
    main()
