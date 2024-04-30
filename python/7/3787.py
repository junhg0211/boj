def main():
    while True:
        try:
            k = int(input())
        except EOFError:
            break

        i = 1
        while i * (i + 1) // 2 < k:
            i += 1

        index = k - i * (i - 1) // 2

        if i % 2 == 0:
            up, down = index, i - index + 1
        else:
            up, down = i - index + 1, index

        print(f"TERM {k} IS {up}/{down}")


if __name__ == "__main__":
    main()
