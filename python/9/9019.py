from sys import stdout, stdin
from collections import deque


input = stdin.readline

d = lambda x: (2 * x) % 10000
s = lambda x: (x - 1) % 10000
l = lambda x: (x * 10 + x // 1000) % 10000
r = lambda x: (x // 10 + x % 10 * 1000) % 10000


def tick():
    a, b = map(int, input().split())

    queue = deque([(a, '')])
    been = {a}
    while queue:
        now, history = queue.popleft()
        # print(f'  {now:4} {history} ({len(queue)} / {len(been)})')

        if now == b:
            stdout.write(f'{history}\n')
            return

        if (result := d(now)) not in been:
            queue.append((result, history + 'D'))
            been.add(result)

        if (result := s(now)) not in been:
            queue.append((result, history + 'S'))
            been.add(result)

        if (result := l(now)) not in been:
            queue.append((result, history + 'L'))
            been.add(result)

        if (result := r(now)) not in been:
            queue.append((result, history + 'R'))
            been.add(result)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
