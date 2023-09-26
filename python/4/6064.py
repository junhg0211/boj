def eea(s: int, t: int) -> tuple[int, int]:
    """ tx === 1 mod s. returns x """

    q, r = divmod(s, t)

    if r == 0:
        return 1, 0

    a1, a2 = eea(t, r)
    a = (a2 - q*a1) % s
    return a, a1


def gcd(a: int, b: int) -> int:
    if b > a:
        return gcd(b, a)

    if b == 0:
        return a

    return gcd(b, a%b)


def calculate(m, n, x, y):
    # j = ma + x = nb + y

    # ma+x === y mod n
    # ma === y-x mod n
    g = gcd(gcd(m, (y-x)%n), n)
    mu = m // g
    xi = (y-x)%n // g
    nu = n // g

    # mu a === xi mod nu
    # mu i === 1 mod nu
    i = eea(nu, mu)[0] % nu
    a = i * xi
    # j === ma + x mod lcm(n,m)
    lcm = n*m // gcd(n, m)
    j = (m*a + x) % lcm

    return j


def tick():
    m, n, x, y = map(int, input().split())

    x -= 1
    y -= 1

    j1 = calculate(m, n, x, y)
    j2 = calculate(n, m, y, x)

    if (j1 - x) % m == (j1 - y) % n == 0:
        return print(j1 + 1)
    if (j2 - x) % m == (j2 - y) % n == 0:
        return print(j2 + 1)
    print(-1)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
