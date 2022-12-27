def trinum(n):
    return n * (n+1) // 2


def main():
    n = int(input())

    result = 0
    for i in range(n):
        delta = trinum(n - i)
        result += delta
        if i % 2:
            result += delta

    print(result)


if __name__ == '__main__':
    main()
