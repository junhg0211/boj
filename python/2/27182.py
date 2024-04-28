def main():
    n, m = map(int, input().split())

    if m < n:
        print((n + m) // 2)
    elif n - 7 >= 1:
        print(n - 7)
    else:
        print(m + 7)


if __name__ == "__main__":
    main()
