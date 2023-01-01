from collections import deque


def main():
    _, length = map(int, input().split())
    holes = deque(sorted(map(int, input().split())))

    tapes = 0
    covered = 0
    while holes:
        start = holes.popleft()
        covered = start + length
        tapes += 1

        while len(holes) > 0 and holes[0] < covered:
            holes.popleft()

    print(tapes)


if __name__ == '__main__':
    main()
