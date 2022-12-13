def main():
    input()
    x, y = 0, 0
    x_min, x_max, y_min, y_max = 0, 0, 0, 0
    direction = 0

    beens = [(x, y)]

    commands = input()
    for command in commands:
        if command == 'L':
            direction = (direction - 1) % 4
            continue

        if command == 'R':
            direction = (direction + 1) % 4
            continue

        if direction == 0:
            y += 1
        elif direction == 1:
            x -= 1
        elif direction == 2:
            y -= 1
        else:
            x += 1

        beens.append((x, y))
        x_max, y_max = max(x, x_max), max(y, y_max)
        x_min, y_min = min(x, x_min), min(y, y_min)

    width, height = x_max - x_min + 1, y_max - y_min + 1
    board = [[False for _ in range(width)] for _ in range(height)]

    for been in beens:
        x, y = been
        x -= x_min
        y -= y_min

        board[y][x] = True

    for y in range(height):
        for x in range(width):
            print('.' if board[y][x] else '#', end='')
        print()


if __name__ == '__main__':
    main()
