def main():
    scoreboard = input()

    a = 0
    b = 0
    rule_a = True
    for i in range(0, len(scoreboard), 2):
        who = scoreboard[i]
        score = int(scoreboard[i+1])

        if who == 'A':
            a += score
        else:
            b += score

        if a == b == 10:
            rule_a = False

        if rule_a:
            if a >= 11:
                return print('A')
            elif b >= 11:
                return print('B')
        else:
            if a >= b + 2:
                return print ('A')
            elif b >= a + 2:
                return print('B')


if __name__ == '__main__':
    main()
