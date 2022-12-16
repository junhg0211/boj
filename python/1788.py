def main():
    n = int(input())
    absolute = abs(n)

    a, b = 1, 0
    for _ in range(absolute):
        c = a + b
        a = b
        b = c % 1_000_000_000

    print(-1 if (n < 0 and n % 2 == 0) else 1 if n else 0)
    print(b)


if __name__ == '__main__':
    main()
