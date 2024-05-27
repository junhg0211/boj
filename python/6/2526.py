def main():
    n, p = map(int, input().split())

    index = {n: 0}
    now = n

    while True:
        now = now * n % p

        if now in index:
            print(len(index) - index[now])
            break

        index[now] = len(index)


if __name__ == "__main__":
    main()
