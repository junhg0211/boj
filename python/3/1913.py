size = int(input())
board = [[0 for _ in range(size)] for _ in range(size)]
x, y = size//2, size//2

number = int(input())

direction = 3  # UP > RIGHT > DOWN > LEFT
double_distance = 2

moved = 0
i = 1
while x or y:
    board[y][x] = i
    if i == number:
        result = y, x

    if not moved:
        moved = double_distance // 2
        double_distance += 1
        direction = (direction + 1) % 4

    if direction == 0:
        y -= 1
    elif direction == 1:
        x += 1
    elif direction == 2:
        y += 1
    else:
        x -= 1

    i += 1
    moved -= 1
board[y][x] = i

if i == number:
    result = y, x

for row in board:
    for value in row:
        # print(format(value, '3d'), end=' ')
        print(value, end=' ')
    print()

print(*map(lambda x: x+1, result))
