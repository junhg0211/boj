height, width = map(int, input().split())
shoe_height, shoe_width = map(int, input().split())

board = [list(map(int, input().split())) for _ in range(height)]

drain_count = int(input())
drains = set(tuple(map(lambda x: int(x)-1, input().split())) for _ in range(drain_count))

# -- find goins
goins = set()

for i in range(height):
    for j in range(width):
        if (i, j) in drains:
            continue

        if i > 0 and (board[i-1][j] <= board[i][j] or (i-1, j) in drains):
            continue

        if j > 0 and (board[i][j-1] <= board[i][j] or (i, j-1) in drains):
            continue

        if i < height-1 and (board[i+1][j] <= board[i][j] or (i+1, j) in drains):
            continue

        if j < width-1 and (board[i][j+1] <= board[i][j] or (i, j+1) in drains):
            continue

        goins.add((i, j))

print(goins)
