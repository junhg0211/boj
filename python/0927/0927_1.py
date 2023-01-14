def main():
    input()

    score = 0
    previous = 0
    for number in map(int, input().split()):
        if previous == number-1:
            previous = number
            continue

        score += number
        previous = number

    print(score)


if __name__ == '__main__':
    main()

