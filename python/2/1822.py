def main():
    input()
    a = set(input().split())
    b = set(input().split())

    c = a - b
    print(len(c))
    if c:
        print(*sorted(map(int, c)), sep=' ')


if __name__ == '__main__':
    main()
