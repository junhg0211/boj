from collections import deque
from sys import stdin

input = stdin.readline


def main():
    computer_count, connection_count = map(int, input().split())
    connection = [set() for _ in range(computer_count)]

    for _ in range(connection_count):
        a, b = map(int, input().split())
        connection[a-1].add(b)

    gots = [0 for _ in range(computer_count)]
    for i in range(1, computer_count+1):
        queue = deque([i])

        while queue:
            now = queue.popleft()

            for connectee in connection[now-1]:
                gots[connectee-1] += 1
                queue.append(connectee)

    max_gots = max(gots)

    for i in range(computer_count):
        if gots[i] == max_gots:
            print(i+1, end=' ')


if __name__ == '__main__':
    main()
