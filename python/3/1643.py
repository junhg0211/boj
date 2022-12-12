from math import log10


def gcd(a: int, b: int):
    if a < b:
        return gcd(b, a)

    c = a - b
    a = b
    b = c

    if b == 0:
        return a

    return gcd(a, b)


def tick(n: int):
    a, b = 0, 1

    for i in range(1, n+1):
        a, b = a*i + n*b, b*i

    g = gcd(a, b)
    a //= g
    b //= g

    return *divmod(a, b), b


def main():
    while True:
        try:
            zahlen, numerator, denominator = tick(int(input()))
        except EOFError:
            break
        except ValueError:
            break

        if numerator == 0:
            print(zahlen)
            continue

        length = int(log10(zahlen))+1

        print(f'{" " * length} {numerator}')
        print(f'{zahlen} {"-" * (int(log10(denominator))+1)}')
        print(f'{" " * length} {denominator}')


if __name__ == '__main__':
    main()
