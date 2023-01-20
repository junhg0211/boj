def main():
    a, b = map(int, input().split())
    c = int(input())

    if a + b < 2 * c:
        print(a + b)
    else:
        print(a + b - 2*c)


if __name__ == '__main__':
    main()
