from sys import stdin

size = int(stdin.readline())

board = list()
for _ in range(size):
    board.append(tuple(map(int, stdin.readline().split())))


def divide(bi):
    x = bi[0]
    y = bi[1]
    ps = bi[2] // 3

    result = list()
    for i in range(3):
        for j in range(3):
            new_x = x + ps*j
            new_y = y + ps*i
            result.append((new_x, new_y, ps))
    return result


def is_innocent(bi):
    for i in range(bi[0], bi[0] + bi[2]):
        for j in range(bi[1], bi[1] + bi[2]):
            if board[i][j] != board[bi[0]][bi[1]]:
                return False
    return True


queue = [(0, 0, size)]
count = [0, 0, 0]

while queue:
    board_info = queue.pop(0)
    if is_innocent(board_info):
        count[board[board_info[0]][board_info[1]]] += 1
        continue

    queue.extend(divide(board_info))

print(count[-1], count[0], count[1], sep='\n')

