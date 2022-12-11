def main():
    _, times = map(int, input().split())

    numbers = list(map(int, input().split(',')))

    for _ in range(times):
        numbers = [numbers[i] - numbers[i-1] for i in range(1, len(numbers))]

    print(','.join(map(str, numbers)))


if __name__ == '__main__':
    main()
