def gcd(a, b):
    if a < b:
        a, b = b, a

    while b > 0:
        a, b = b, a%b

    return a


def tick():
    m, n, x, y = map(int, input().split())

    g = gcd(m, n)

    if (x - y) % g != 0:
        return print(-1)

    while x != y:
        if x < y:
            x += m
        elif x > y:
            y += n

    print(x)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
