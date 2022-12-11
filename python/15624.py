def main():
    n = int(input())

    if n == 0:
        print(0)
        return
    if n == 1:
        print(1)
        return

    a, b = 0, 1
    for _ in range(n-2):
        c = a + b
        a = b
        b = c

    print(c % 1_000_000_007)


if __name__ == '__main__':
    main()
