def main():
    n = int(input())

    if n <= 2:
        print(n)
        return

    a, b = 1, 2
    for _ in range(n - 2):
        a, b = b, (a+b) % 10

    print(b)

if __name__ == '__main__':
    main()
