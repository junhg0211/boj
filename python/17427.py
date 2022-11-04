from math import sqrt


def f(n):
    prime = 2
    divisors = dict()
    while n > 1:
        while n % prime == 0:
            n //= prime
            if prime in divisors:
                divisors[prime] += 1
            else:
                divisors[prime] = 1
        prime += 1

    result = 0


def main():
    n = int(input())

    print(f(n))


if __name__ == '__main__':
    main()
