def main():
    start = int(input())

    for i in range(start, int(input()) + 1):
        if (i - start) % 60 == 0:
            print('All positions change in year', i)


if __name__ == '__main__':
    main()
