def main():
    while True:
        a, b = map(int, input().split())

        if a == 0 and b == 0:
            return

        print(a + b)


if __name__ == '__main__':
    main()
