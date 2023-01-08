letter_count = dict()
been = dict()


def get_index(letter: str):
    mae = sum(map(lambda x: x[1], filter(lambda x: x[0] < letter, letter_count.items())))
    result = mae + been[letter]
    been[letter] += 1
    return result


def main():
    letter_count.clear()
    been.clear()

    key = input()
    key_len = len(key)
    for letter in key:
        if letter in letter_count:
            letter_count[letter] += 1
        else:
            letter_count[letter] = 1
            been[letter] = 0

    encrypted = input()
    encrypted_len = len(encrypted)
    password_board = list()
    while encrypted:
        password_board.append(encrypted[:encrypted_len // key_len])
        encrypted = encrypted[encrypted_len // key_len:]

    string_board = list()
    for letter in key:
        string_board.append(password_board[get_index(letter)])

    for i in range(encrypted_len // key_len):
        for j in range(key_len):
            print(string_board[j][i], end='')
    print()


if __name__ == '__main__':
    main()
