from decimal import Decimal, getcontext, ROUND_HALF_UP
from math import factorial

PREC = 100
getcontext().prec = PREC
getcontext().rounding = ROUND_HALF_UP

PI = Decimal('3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446')
D_PI = Decimal(2) * PI
smol = Decimal('0.' + '0'*(PREC-1) + '1')


def sin(x: Decimal):
    x %= D_PI
    result = Decimal(0)
    term = Decimal(1)
    n = 0
    while term > smol:
        term = x ** Decimal(2*n + 1) / Decimal(factorial(2*n + 1))
        if n % 2 == 0:
            result += term
        else:
            result -= term
        n += 1
    return result


def main():
    a, b, c = map(Decimal, input().split())

    start = c/a - PI
    end = c/a + PI

    while end - start > Decimal('0.00000001'):
        middle = (end + start) / Decimal(2)
        value = a*middle + b*sin(middle)

        if value > c:
            end = middle
        else:
            start = middle

    print(middle)
    print(round(middle, 6))


if __name__ == '__main__':
    main()
