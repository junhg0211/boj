def main():
    print('Gnomes:')

    for _ in range(int(input())):
        a, b, c = map(int, input().split())

        if a <= b <= c or a >= b >= c:
            print('Ordered')
        else:
            print('Unordered')


if __name__ == '__main__':
    main()
