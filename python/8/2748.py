def main():
    n = int(input())

    a, b = 1, 0
    for _ in range(n):
        c = a + b
        a = b
        b = c

    print(b)


if __name__ == '__main__':
    main()
