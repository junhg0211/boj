def multiply_all(number: str):
    result = 1
    for letter in number:
        result *= int(letter)
    return result


def is_yujinsu(number: int):
    for i in range(1, len(number)):
        if multiply_all(number[:i]) == multiply_all(number[i:]):
            return True
    return False


def main():
    number = input()

    if is_yujinsu(number):
        print('YES')
    else:
        print('NO')


if __name__ == '__main__':
    main()
