def process(string) -> int:
    # print('i', string)

    previous = ''
    multiplier = 0
    opening = 0
    result = 0
    buffer = ''
    for letter in string:
        if letter == '(':
            if opening == 0:
                multiplier = int(previous)
                result -= 1
            else:
                buffer += '('
            opening += 1
        elif letter == ')':
            opening -= 1
            if opening == 0:
                result += multiplier * process(buffer)
                buffer = ''
                # print('m', multiplier)
            else:
                buffer += ')'
        else:
            if opening:
                buffer += letter
            else:
                result += 1

        previous = letter

    # print('r', result)
    return result


def main():
    string = input()
    print(process(string))


if __name__ == '__main__':
    main()
