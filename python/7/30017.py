def main():
    a, b = map(int, input().split())

    print(min(a - 1, b) * 2 + 1)


if __name__ == "__main__":
    main()
