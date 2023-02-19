def tick():
    input()
    previous_time = 0
    walking = 0
    for time in map(int, input().split()):
        walking += (time - previous_time) // 120
        previous_time = time
    walking += (1440 - time) // 120

    if walking >= 2:
        print('YES')
    else:
        print('NO')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
