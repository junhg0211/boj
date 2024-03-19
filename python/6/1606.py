def main():
    ox, oy = map(int, input().split())

    if ox == 0 and oy == 0:
        print(1)
        return

    x = ox
    y = oy
    level = 0
    if x < 0 and y > 0:
        diff = min(-x, y)
        x += diff
        y -= diff
        level += diff
    elif x > 0 and y < 0:
        diff = min(x, -y)
        x -= diff
        y += diff
        level += diff
    level += abs(x) + abs(y)

    x = ox
    y = oy
    steps = 0
    if x == level and y == 0:
        steps = level * 6
    elif x >= 0 and y >= 0:
        # print("a")
        steps = y
    elif y == level:
        # print("b")
        steps = level + abs(x)
    elif x == -level:
        # print("c")
        steps = level * 2 + level - y
    elif x <= 0 and y <= 0:
        # print("d")
        steps = level * 3 + -y
    elif y == -level:
        # print("e")
        steps = level * 4 + x
    # elif x == level:
    else:
        print("f")
        steps = level * 5 + level + y

    # print("leve", level)
    # print("step", steps)

    print(3 * (level - 1) * level + 1 + steps)


if __name__ == "__main__":
    main()
