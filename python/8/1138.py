def main():
    number = int(input())

    order = list()
    for i, count in enumerate(map(int, reversed(input().split()))):
        now = number-i

        order.insert(count, str(now))

    print(' '.join(order))


if __name__ == '__main__':
    main()
