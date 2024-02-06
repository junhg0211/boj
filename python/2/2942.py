def get_gcd(a: int, b: int):
    if a > b:
        return get_gcd(b, a)

    while True:
        a, b = b%a, a
        if a == 0:
            return b


def get_divisors(number: int):
    stack = list()
    for i in range(1, int(number**0.5) + 1):
        if number % i == 0:
            yield i
            compliment = number // i
            if compliment != i:
                yield compliment



def main():
    a, b = map(int, input().split())

    gcd = get_gcd(a, b)

    for divisor in get_divisors(gcd):
        print(f'{divisor} {a // divisor} {b // divisor}')


if __name__ == '__main__':
    main()
