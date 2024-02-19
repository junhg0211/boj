def main():
    a, b = map(int, input().split())

    for i in range(2, b):
        if a % i == 0:
            return print(f'BAD {i}')

    print('GOOD')


if __name__ == '__main__':
    main()
