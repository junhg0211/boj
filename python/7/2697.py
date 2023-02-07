from sys import stdin

input = stdin.readline


def tick():
    number = list(input().rstrip())

    reverse = list()
    anchor = ''
    i = 0
    while number:
        letter = number.pop()

        if reverse and letter < reverse[-1]:
            anchor = letter
            reverse.append(anchor)
            break

        reverse.append(letter)

    if anchor:
        number = ''.join(number)
        print(number, end='')

        for i, letter in enumerate(reverse):
            if letter > anchor:
                print(reverse.pop(i), end='')
                break

        print(''.join(sorted(reverse)))
    else:
        print('BIGGEST')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
