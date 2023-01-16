def main():
    possibles = input()
    dice = tuple(sorted(map(int, input().split())))

    straight_combination = (
        (1, 2, 3), (1, 2, 4), (1, 2, 5),
        (1, 3, 4), (1, 3, 5), (1, 4, 5),
        (2, 3, 4), (2, 3, 5), (2, 4, 5), (3, 4, 5)
    )

    max_score = 0

    for i in range(1, 7):
        # numbers
        if possibles[i-1] == 'Y':
            score = (dice.count(i)+2) * i
            max_score = max(max_score, score)

        if dice.count(i) >= 2:
            # foak
            if possibles[6] == 'Y':
                score = 4 * i
                max_score = max(max_score, score)

            # full house
            if possibles[7] == 'Y':
                if dice[0] != dice[2]:
                    score = dice[0]*2 + dice[2]*3
                else:
                    if dice[0] != 6:
                        score = sum(dice) + 12
                    else:
                        score = sum(dice) + 10
                max_score = max(max_score, score)

        # yacht
        if dice.count(i) == 3:
            if possibles[10] == 'Y':
                score = 50
                max_score = max(max_score, score)

    # little
    if possibles[8] == 'Y':
        if dice in straight_combination:
            score = 30
            max_score = max(max_score, score)

    # big
    if possibles[9] == 'Y':
        if tuple(map(lambda x: x-1, dice)) in straight_combination:
            score = 30
            max_score = max(max_score, score)

    # choice
    if possibles[11] == 'Y':
        score = sum(dice) + 12
        max_score = max(max_score, score)

    print(max_score)


if __name__ == '__main__':
    main()
