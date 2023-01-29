def main():
    a, b, c = map(int, input().split())
    a, b, c = sorted([a, b, c])

    print(c-b + c-a)


if __name__ == '__main__':
    main()
