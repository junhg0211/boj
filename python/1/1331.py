def get_place(went):
    place = input()
    position = ('ABCDEF'.index(place[0]), int(place[1]) - 1)
    if went[position[0] * 6 + position[1]] == 1:
        return 1
    went[position[0] * 6 + position[1]] = 1
    return position


def is_valid_movement(a, b):
    c = abs(a[0] - b[0])
    d = abs(a[1] - b[1])
    return sorted((c, d)) == [1, 2]


def main():
    went = [0 for _ in range(6*6)]

    start = previous = get_place(went)
    for _ in range(35):
        now = get_place(went)
        if now == 1:
            print('Invalid')
            return
        if not is_valid_movement(previous, now):
            print('Invalid')
            return
        previous = now
    if not is_valid_movement(now, start):
        print('Invalid')
        return
    print('Valid')


if __name__ == '__main__':
    main()
