gandalph_weight = [1, 2, 3, 3, 4, 10]
sauron_weight = [1, 2, 2, 2, 3, 5, 10]


def tick(round_):
    gandalph = list(map(int, input().split()))
    gandalph_score = 0
    for i in range(len(gandalph)):
        gandalph_score += gandalph[i] * gandalph_weight[i]

    sauron = list(map(int, input().split()))
    sauron_score = 0
    for i in range(len(sauron)):
        sauron_score += sauron[i] * sauron_weight[i]

    if gandalph_score > sauron_score:
        print(f'Battle {round_}: Good triumphs over Evil')
    elif gandalph_score < sauron_score:
        print(f'Battle {round_}: Evil eradicates all trace of Good')
    else:
        print(f'Battle {round_}: No victor on this battle field')


def main():
    testcase_count = int(input())
    for i in range(1, testcase_count + 1):
        tick(i)


if __name__ == '__main__':
    main()
