from math import ceil


debug = False


def is_valid_anchor(string: str, anchor: int) -> bool:
    even = 0 if anchor % 1 else 1
    anchor = int(anchor)

    string_length = len(string)
    for i in range(1, anchor+1):
        debug and print(1, i)
        if anchor + i - even >= string_length:
            return True
        if string[anchor - i] != string[anchor + i - even]:
            return False
    return True


def main():
    string = input()

    i = len(string) / 2
    while True:
        debug and print(0, i)
        if is_valid_anchor(string, i):
            break
        i += 0.5

    print(int(i * 2))


if __name__ == '__main__':
    main()
