def main():
    numbers = tuple(map(int, input().split()))

    print(abs(numbers[0] + numbers[3] - numbers[1] - numbers[2]))


if __name__ == '__main__':
    main()
