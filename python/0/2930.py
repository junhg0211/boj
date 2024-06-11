def get_score(me, them):
    me = 'RSP'.index(me)
    them = 'RSP'.index(them)

    if me == them:
        return 1

    if (me - them) % 3 == 2:
        return 2

    return 0


def main():
    round_count = int(input())

    sanggeun = input()
    friend_count = int(input())
    board = list()
    for _ in range(friend_count):
        board.append(input())

    now_score = 0
    max_score = 0
    for i in range(round_count):
        score = [0, 0, 0]
        for j in range(friend_count):
            now_score += get_score(sanggeun[i], board[j][i])

            for k in range(3):
                score[k] += get_score('RSP'[k], board[j][i])

        max_score += max(score)

    print(now_score)
    print(max_score)


if __name__ == '__main__':
    main()
