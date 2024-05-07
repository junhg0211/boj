def main():
    while True:
        a, b, c = map(int, input().split())

        if a == b == c == 0:
            break

        if 2 * b == c + a:
            print("AP", c + b - a)
        else:
            print("GP", c * b // a)


if __name__ == "__main__":
    main()
