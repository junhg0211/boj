def main():
    a, b, c = map(int, input().split())

    result = 1
    i = a
    while b:
        if b & 1:
            result *= i
            result %= c
        b >>= 1
        i *= i
        i %= c

    print(result % c)


if __name__ == '__main__':
    '''
    from io import StringIO
    input = StringIO('2147483647 123 1233123123').readline
    '''
    main()
