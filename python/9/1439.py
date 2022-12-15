def oneize(string):
    previous = '1'
    count = 0
    for letter in string:
        if letter == '0' and previous == '1':
            count += 1
        previous = letter
    return count


def zeroize(string):
    previous = '0'
    count = 0
    for letter in string:
        if letter == '1' and previous == '0':
            count += 1
        previous = letter
    return count


def main():
    string = input()

    print(min(oneize(string), zeroize(string)))


if __name__ == '__main__':
    main()
