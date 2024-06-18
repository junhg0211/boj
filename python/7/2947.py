def main():
    numbers = list(map(int, input().split()))

    while numbers != [1, 2, 3, 4, 5]:
        for i in range(4):
            if numbers[i] > numbers[i+1]:
                numbers[i], numbers[i+1] = numbers[i+1], numbers[i]

                print(' '.join(map(str, numbers)))


if __name__ == '__main__':
    main()
