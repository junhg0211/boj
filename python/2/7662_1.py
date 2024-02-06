import heapq
from sys import stdin

input = stdin.readline


def tick():
    min_queue = list()
    max_queue = list()

    valid = dict()
    size = 0

    for _ in range(int(input())):
        command, args = input().split()
        args = int(args)

        if command == 'I':
            heapq.heappush(min_queue, args)
            heapq.heappush(max_queue, -args)

            if args in valid:
                valid[args] += 1
            else:
                valid[args] = 1
            size += 1

        elif size == 0:
            continue

        elif args == 1:
            while True:
                remove = -heapq.heappop(max_queue)
                if valid[remove]:
                    valid[remove] -= 1
                    size -= 1
                    break

        else:
            while True:
                remove = heapq.heappop(min_queue)
                if valid[remove]:
                    valid[remove] -= 1
                    size -= 1
                    break

        # print(valid, min_queue, max_queue)

    if not size:
        print('EMPTY')
        return

    max_ = None
    min_ = None

    while True:
        max_ = -heapq.heappop(max_queue)
        if valid[max_]:
            break
    while True:
        min_ = heapq.heappop(min_queue)
        if valid[min_]:
            break

    print(max_, min_)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
