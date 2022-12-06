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


def get_width(iteration: int) -> int:
    return 4 * iteration - 3


def get_height(iteration: int) -> int:
    if iteration == 1:
        return 1
    else:
        return 4 * iteration - 1


iterations = int(input())
if iterations == 1:
    print('*')
    exit()

previous_board = None
for i in range(1, iterations):
    board = Board(get_width(i + 1), get_height(i + 1))
    if previous_board is None:
        previous_board = Board(get_width(i), get_height(i))
        previous_board.set(0, 0)
        previous_board.set(0, 1)
        previous_board.set(0, 2)

    board.stamp(2, 2, previous_board)
    for i in range(board.height):
        if i < board.width:
            board.set(i, 0)
            board.set(i, board.height - 1)
        board.set(0, i)
        if i != 1:
            board.set(board.width - 1, i)
    board.set(board.width - 2, 2)
    previous_board = board

print(previous_board)
