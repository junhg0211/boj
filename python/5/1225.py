def main():
    a, b = input().split()

    result = 0
    for i in map(int, a):
        for j in map(int, b):
            result += i*j

    print(result)


if __name__ == '__main__':
    main()
