from sys import stdin

input = stdin.readline


def main():
    input()

    volume = 0.0
    for side in map(float, input().split()):
        volume += side**3

    print(volume**(1/3))


if __name__ == '__main__':
    main()
