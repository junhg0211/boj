def main():
    str_, dex, int_, luk, n = map(int, input().split())

    print(max(0, n * 4 - (str_ + dex + int_ + luk)))


if __name__ == "__main__":
    main()
