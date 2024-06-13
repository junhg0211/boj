def main():
    count = int(input())
    numbers = list()
    for _ in range(count):
        numbers.append(int(input()))

    sum_ = sum(numbers)
    for i in range(count):
        if sum_ - numbers[i] == numbers[i]:
            print(numbers[i])
            return

    print('BAD')


if __name__ == '__main__':
    main()
