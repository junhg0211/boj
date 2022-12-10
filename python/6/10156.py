def main():
    a, b, c = map(int, input().split())

    print(max(0, a * b - c))


if __name__ == '__main__':
    main()
