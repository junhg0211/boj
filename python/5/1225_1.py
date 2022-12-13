def main():
    a, b = input().split()
    print(sum(map(int, a)) * sum(map(int, b)))


if __name__ == '__main__':
    main()
