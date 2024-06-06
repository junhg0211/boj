def main():
    a = int(input())

    for n in range(1, a + 1):
        if not (0 < 30 % (n + 1) <= n):
            print(n)


if __name__ == "__main__":
    main()
