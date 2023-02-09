def main():
    positions = list(map(int, input().split()))

    x, y, r = map(int, input().split())

    if x in positions:
        print(positions.index(x) + 1)
    else:
        print('0')


if __name__ == '__main__':
    main()
