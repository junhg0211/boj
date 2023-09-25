def gcd(a, b):
    if b > a:
        return gcd(b, a)

    if b == 0:
        return a

    return gcd(b, a%b)


def tick():
    m, n, x, y = map(int, input().split())
    x -= 1
    y -= 1

    lcm = m*n // gcd(m, n)
    for i in range(lcm):
        if (i-x) % m == (i-y) % n == 0:
            print(i + 1)
            return

    print(-1)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
