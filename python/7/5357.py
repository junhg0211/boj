def tick():
    previous = ''
    for letter in input():
        if letter != previous:
            print(letter, end='')
        previous = letter
    print()


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
