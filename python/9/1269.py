def main():
    input()
    a = set(map(int, input().split()))
    b = set(map(int, input().split()))

    print(len(a-b) + len(b-a))


if __name__ == '__main__':
    main()
