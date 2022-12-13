def main():
    count, aim = map(int, input().split())

    now = 0
    numbers = list()
    for number in map(int, input().split()):
        now += number
        numbers.append(now)

    numbers.append(now)

    result = 0
    for i in range(count):
        for j in range(i, count+1):
            if numbers[j] - numbers[i] == aim:
                result += 1

    print(result)


if __name__ == '__main__':
    main()
