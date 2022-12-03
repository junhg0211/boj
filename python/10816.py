from sys import stdin

input = stdin.readline


def main():
    cards = dict()

    input()
    for card in input().split():
        if card in cards:
            cards[card] += 1
        else:
            cards[card] = 1

    input()
    for test in input().split():
        print(cards.get(test, 0), end=' ')
    print()


if __name__ == '__main__':
    main()
