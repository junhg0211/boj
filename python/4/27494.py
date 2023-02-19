def is_valid(number: int):
    number = str(number)

    targets = ['2', '0', '2', '3']
    for letter in number:
        if not targets:
            return True
        if letter == targets[0]:
            targets.pop(0)

    return not targets


def main():
    print(sum(map(int, (is_valid(i) for i in range(int(input()) + 1)))))


if __name__ == '__main__':
    main()
