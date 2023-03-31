from collections import deque


def main():
    cards = deque(range(1, int(input()) + 1))

    i = 0
    while cards:
        if i % 2 == 0:
            print(cards.popleft(), end=' ')
        else:
            cards.append(cards.popleft())

        i += 1


if __name__ == '__main__':
    main()
