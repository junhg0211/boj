def tick():
    size = int(input())
    board = [input() for _ in range(size)]

    for i in range(size, -1, -1):
        for y in range(size-i+1):
            for x in range(size-i+1):
                vacant = True
                for j in range(i):
                    no = False
                    for k in range(i):
                        if board[y+j][x+k] == '#':
                            vacant = False
                            no = True
                            break
                    if no:
                        break

                if vacant:
                    print(i)
                    return


def main():
    testcase_count = int(input())

    for _ in range(testcase_count):
        tick()


if __name__ == '__main__':
    main()
