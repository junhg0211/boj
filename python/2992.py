def main():
    number = input()

    yes = False
    for i in range(len(number)-1, 0, -1):
        if number[i-1] < number[i]:
            yes = True
            break

    if not yes:
        print(0)
        return

    numbers = list(number[i-1:])
    numbers.pop(1)

    number = number[:i-1] + number[i] + ''.join(sorted(numbers))
    print(number)


if __name__ == '__main__':
    main()
