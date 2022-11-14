def main():
    numbers = sorted(map(int, input().split()))

    if numbers[0] == numbers[1] == numbers[2]:
        print(10000 + numbers[0] * 1000)
        return

    if numbers[2] == numbers[1] or numbers[0] == numbers[1]:
        print(numbers[1] * 100 + 1000)
        return
    elif numbers[0] == numbers[2]:
        print(numbers[0] * 100 + 1000)
        return

    print(numbers[2] * 100)


if __name__ == '__main__':
    main()
