from sys import stdin
from math import inf

input = stdin.readlie


def main():
    bell_count, home_count, strength = map(int, input().split())
    bell_positions = tuple(map(int, input().split()))
    home_positions = map(int, input().split())

    for home in home_positions:
        distance = inf
        volume = 0
        for bell in bell_positions:
            if (new_distance := abs(bell - home)) > distance:
                break
            distance = new_distance

            volume = max(volume, max(strength - distance, 0))

        print(volume)


if __name__ == '__main__':
    main()
