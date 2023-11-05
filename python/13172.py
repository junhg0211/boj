DIVISOR = 10


def eea(t, s):
    q, r = divmod(s, t)
    if not r:
        return 1, 0

    ap, app = eea(r, t)
    a = (app - q*ap) % s
    return a, ap


def invmod(of, when):
    return eea(of, when)[0]


def gcd(a, b):
    if a < b:
        return gcd(b, a)
    while b:
        a, b = b, a%b
    return a


def fraction_add(a, b):
    gcd_ = gcd(a[1], b[1])
    numerator = a[0]*b[1] // gcd_ + b[0]*a[1] // gcd_
    denominator = a[1]*b[1] // gcd_
    regcd = gcd(numerator, denominator)

    return numerator//regcd, denominator//regcd


def main():
    dice_count = int(input())

    result = (0, 1)

    for _ in range(0, dice_count):
        divisor, sum_ = map(int, input().split())
        fraction = (sum_, divisor)
        result = fraction_add(result, fraction)

    result = result[0] * invmod(result[1], DIVISOR) % DIVISOR
    print(result)


if __name__ == '__main__':
    main()
