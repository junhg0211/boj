def main():
    c, d = map(int, input().split())

    if (d + c) % 2 == 1:
        print('-1')
        return

    a = (d + c) // 2
    b = c - (d + c) // 2

    if a < 0 or b < 0:
        print('-1')
        return

    print(a, b)


if __name__ == '__main__':
    main()
