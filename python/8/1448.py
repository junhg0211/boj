from sys import stdin

input = stdin.readline


def main():
    numbers = sorted(int(input()) for _ in range(int(input())))
    sides = list()

    while True:
        if len(sides) < 3:
            if not numbers:
                print('-1')
                break

            sides.append(numbers.pop())
            continue

        if sides[0] >= sides[1] + sides[2]:
            sides.pop(0)
            continue

        print(sum(sides))
        break


if __name__ == '__main__':
    main()
