from sys import stdout


class Board:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.board = set()

    def __str__(self):
        string = ''
        for y in range(self.height):
            for x in range(self.width):
                string += '*' if (x, y) in self.board else ' '
            string = string.rstrip() + '\n'
        return string

    def set(self, x: int, y: int):
        self.board.add((x, y))

    def stamp(self, x: int, y: int, what: 'Board'):
        for i, j in what.board:
            self.set(x + i, y + j)


iterations = int(input())


def f(x, previous_board=None):
    board = Board(2**(x+1) - 3, 2**x - 1)
    if x == 1:
        board = Board(1, 1)
        board.set(0, 0)
    elif x % 2 == 0:
        for i in range(board.width):
            board.set(i, 0)
            if i < board.height - 1:
                board.set(i+1, i+1)
                board.set(board.width - i - 2, i+1)
        board.stamp(board.width // 4 + 1, 1, previous_board)
    else:
        for i in range(board.width):
            board.set(i, board.height - 1)
            if i < board.height - 1:
                board.set(board.width // 2 - i, i)
                board.set(board.width - board.width // 2 + i - 1, i)
        board.stamp(board.width // 4 + 1, board.height // 2, previous_board)
    return board


result = None
for iteration in range(iterations):
    result = f(iteration + 1, result)

stdout.write(str(result))
