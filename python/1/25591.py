def main():
    m, n = map(int, input().split())

    a = 100 - m
    b = 100 - n
    c = 100 - (a + b)
    d = a * b
    q = d // 100
    r = d % 100

    print(a, b, c, d, q, r)
    print(c + q, r)


if __name__ == '__main__':
    main()
