height, width = map(int, input().split())
board = [list(map(int, input().split())) for _ in range(height)]


def get_area(x, y):
    result = 1
    board[y][x] = 0
    if x > 0 and board[y][x-1] == 1: result += get_area(x-1, y)
    if y > 0 and board[y-1][x] == 1: result += get_area(x, y-1)
    if x < width - 1 and board[y][x+1] == 1: result += get_area(x+1, y)
    if y < height - 1 and board[y+1][x] == 1: result += get_area(x, y+1)
    return result


max_area = 0
count = 0

for i in range(height):
    for j in range(width):
        if board[i][j] == 1:
            area = get_area(j, i)
            if area > max_area:
                max_area = area
            count += 1

print(count)
print(max_area)

