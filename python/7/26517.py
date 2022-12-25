def main():
    k = int(input())
    a, b, c, d = map(int, input().split())

    if a*k+b == c*k+d:
        print('Yes', a*k+b)
    else:
        print('No')


if __name__ == '__main__':
    main()
