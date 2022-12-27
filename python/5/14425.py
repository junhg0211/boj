def main():
    n, m = map(int, input().split())

    s = set()
    for _ in range(n):
        s.add(input())

    count = 0
    for _ in range(m):
        if input() in s:
            count += 1

    print(count)


if __name__ == '__main__':
    main()
