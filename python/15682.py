from sys import stdin
from math import sqrt, ceil

input = stdin.readline


def get_divisors(number: int) -> list[int]:
    number = abs(number)
    bigs = [number]
    smalls = [1]

    for i in range(2, ceil(sqrt(number)) + 1):
        if number % i == 0:
            if (big := number // i) != i:
                bigs.append(big)
            smalls.append(i)

    smalls.extend(bigs[::-1])
    return smalls


def get_candidates(a: int, d: int) -> int:
    adivs = get_divisors(a)
    ddivs = get_divisors(d)

    already_returned = dict()

    for adiv in adivs:
        for ddiv in ddivs:
            candidate = ddiv / adiv
            if (result := int(candidate)) == candidate and result not in already_returned:
                already_returned[result] = True
                yield candidate
                yield -candidate


def synthetic_division(a, b, c, d, t) -> tuple[int, int]:
    qb = a*t + b
    qc = qb*t + c

    if qc*t + d:
        raise ValueError

    return qb, qc


def gcd(a, b):
    if a < b:
        a, b = b, a
    while b:
        a, b = b, a % b
    return a


def clean_up(a, b, c, d):
    zeros = max(len(str(a%1)), len(str(b%1)), len(str(c%1)), len(str(d%1))) - 2
    multiplier = 10**zeros
    return map(lambda x: x * multiplier, (a, b, c, d))


def tick():
    a, b, c, d = map(float, input().split())
    a, b, c, d = clean_up(a, b, c, d)

    qb, qc = None, None
    for candidate in get_candidates(a, d):
        try:
            qb, qc = synthetic_division(a, b, c, d, candidate)
        except ValueError:
            pass
        else:
            break

    determinant = qb**2 - 4*a*qc

    if determinant < 0:
        print(candidate)
        return

    if determinant == 0:
        x = -qb / (2*a)
        print(' '.join(map(str, sorted({candidate, x}))))
        return

    x1 = (-qb + sqrt(determinant)) / (2*a)
    x2 = (-qb - sqrt(determinant)) / (2*a)

    xs = sorted({x1, x2, candidate})
    print(' '.join(map(str, xs)))


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()

