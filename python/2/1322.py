def main():
    x, k = map(int, input().split())

    nos = set()
    i = 0
    while x:
        if x & 1:
            nos.add(i)
        x >>= 1
        i += 1

    result = 0
    i = 0
    while k:
        while i in nos:
            i += 1
        if k & 1:
            result |= 1 << i
        k >>= 1
        i += 1

    print(result)


if __name__ == '__main__':
    main()
