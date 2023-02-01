from sys import stdin

input = stdin.readline

first_word = None


def process_word(word: str):
    global first_word

    if len(word) < 2:
        return

    if first_word is None or word < first_word:
        first_word = word


def main():
    height, width = map(int, input().split())
    board = tuple(input().rstrip() for _ in range(height))

    for row in board:
        word = ''
        for j, letter in enumerate(row):
            if letter != '#':
                word += letter
            else:
                process_word(word)
                word = ''

            if j == width-1:
                process_word(word)

    for i in range(width):
        word = ''
        for j in range(height):
            letter = board[j][i]

            if letter != '#':
                word += letter
            else:
                process_word(word)
                word = ''

            if j == height-1:
                process_word(word)

    if first_word:
        print(first_word)


if __name__ == '__main__':
    main()
