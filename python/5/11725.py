from collections import deque
from sys import stdin

input = stdin.readline


def main():
    node_count = int(input())
    connection = {i: set() for i in range(1, node_count+1)}

    for _ in range(node_count-1):
        a, b = map(int, input().split())
        connection[a].add(b)
        connection[b].add(a)

    parents = dict()

    queue = deque((1,))
    while queue:
        now = queue.popleft()
        for connected in connection[now]:
            if connected in parents:
                continue

            parents[connected] = now
            queue.append(connected)

    parents.pop(1)

    print('\n'.join(map(lambda x: str(x[1]), sorted(parents.items()))))


if __name__ == '__main__':
    main()
