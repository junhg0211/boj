from copy import copy
from math import inf


def get_kevin(starter, connection):
    connection = copy(connection)
    queue = [(0, starter)]
    result = 0
    while queue:
        history, now = queue.pop(0)
        if now in connection:
            result += history
            queue.extend(map(lambda x: (history+1, x), connection.pop(now)))
    return result


def main():
    nodes, edges = map(int, input().split())

    connection = dict()
    for _ in range(edges):
        a, b = map(int, input().split())
        if a in connection:
            connection[a].append(b)
        else:
            connection[a] = [b]
        if b in connection:
            connection[b].append(a)
        else:
            connection[b] = [a]

    minimum = inf
    lowest_one = None
    for i in range(1, nodes+1):
        if (this := get_kevin(i, connection)) < minimum:
            minimum = this
            lowest_one = i

    print(lowest_one)


if __name__ == '__main__':
    main()
