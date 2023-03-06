def main():
    a, b, c = map(float, input().split())

    if a != c:
        print('0')
        return

    print(format(a * (b+c) * 2, '.4f'))


if __name__ == '__main__':
    main()
