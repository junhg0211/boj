def key(x: str) -> tuple:
    number_sum = 0
    for letter in x:
        if letter in '1234567890':
            number_sum += int(letter)

    return (len(x), number_sum, x)


def main():
    count = int(input())
    names = [input() for _ in range(count)]

    for name in sorted(names, key=key):
        print(name)


if __name__ == '__main__':
    main()
