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
size = lambda x: 4 * x - 3

previous_board = None

for iteration in range(iterations):
    size_ = size(iteration+1)
    board = Board(size_, size_)
    for i in range(size_):
        board.set(i, 0)
        board.set(0, i)
        board.set(i, size_ - 1)
        board.set(size_ - 1, i)
    if previous_board is not None:
        board.stamp(2, 2, previous_board)
    previous_board = board

print(previous_board)
