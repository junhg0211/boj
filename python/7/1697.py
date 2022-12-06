from collections import deque


def tick():
    start, end = map(int, input().split())

    seconds = 0
    queue = deque([start])
    tomorrow_queue = deque()
    been = [False for _ in range(100001)]

    while True:
        while queue:
            position = queue.popleft()

            if position == end:
                return seconds

            if been[position]:
                continue

            been[position] = True
            if position > 0:
                tomorrow_queue.append(position - 1)
            if position < 100_000:
                tomorrow_queue.append(position + 1)
            if 0 < position <= 50_000:
                tomorrow_queue.append(2 * position)

        seconds += 1
        queue = tomorrow_queue
        tomorrow_queue = deque()


def main():
    print(tick())


if __name__ == '__main__':
    main()
