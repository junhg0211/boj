import re

input_re = re.compile(r'^R(\d+)C(\d+)$')

def get_numbers():
    return tuple(map(int, input_re.findall(input())[0]))


characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
char_count = len(characters)


def convert(mod):
    result = ''
    while mod:
        mod -= 1
        result = characters[mod % char_count] + result
        mod //= char_count
    return result


def main():
    while True:
        r, c = get_numbers()

        if not (r or c):
            break

        print(convert(c), r, sep='')


if __name__ == '__main__':
    main()
