def main():
    a, b = map(int, input().split())
    c, d = map(int, input().split())

    print(min(a + d, c + b))


if __name__ == '__main__':
    main()
