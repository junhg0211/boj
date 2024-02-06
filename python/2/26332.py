def tick():
    a, b = map(int, input().split())

    print(a, b)

    if a > 1:
        price = a * (b - 2) + 2
    else:
        price = b

    print(price)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
