from math import inf


def main():
    # getting board
    count = int(input())

    boards = list()
    for _ in range(count):
        boards.append([input() for _ in range(5)])

    # calculating diff
    min_diff = inf
    min_info = ''

    for i in range(count):
        for j in range(i+1, count):
            diff = 0
            for k in range(5):
                for l in range(7):
                    if boards[i][k][l] != boards[j][k][l]:
                        diff += 1

            if diff < min_diff:
                min_diff = diff
                min_info = f'{i+1} {j+1}'

    # print result
    print(min_info)


if __name__ == '__main__':
    main()
