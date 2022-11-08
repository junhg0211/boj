def main():
    count = int(input())

    numbers = [int(input()) for _ in range(count)]

    for number in numbers:
        print('NO' if any(number % i == 0 for i in range(2, 10**6 + 1)) else 'YES')


if __name__ == '__main__':
    main()
