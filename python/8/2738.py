def main():
    n, m = map(int, input().split())

    array = [[0 for _ in range(m)] for _ in range(n)]

    for _ in range(2):
        for i in range(n):
            for j, number in enumerate(map(int, input().split())):
                array[i][j] += number


    for i in range(n):
        for j in range(m):
            print(array[i][j], end=' ')
        print()


if __name__ == '__main__':
    main()
