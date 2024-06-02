def main():
    input()

    previous = 0
    highest = 0
    now = 0
    for number in map(int, input().split()):
        if previous == 0:
            previous = number

        if previous < number:
            now += number - previous
            highest = max(highest, now)
        else:
            now = 0

        previous = number

    print(highest)


if __name__ == "__main__":
    main()
