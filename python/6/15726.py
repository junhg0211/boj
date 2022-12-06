def main():
    a, b, c = map(int, input().split())

    x = a * c // b
    y = a * b // c

    print(max(x, y))


if __name__ == '__main__':
    main()
