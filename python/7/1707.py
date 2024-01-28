from collections import deque
from sys import stdin

input = stdin.readline

edges = dict()


def tick():
    vertex_count, edge_count = map(int, input().split())

    # -- get connections
    edges.clear()
    for _ in range(edge_count):
        u, v = map(int, input().split())

        if u not in edges:
            edges[u] = set()
        edges[u].add(v)

        if v not in edges:
            edges[v] = set()
        edges[v].add(u)

    # -- check non cycle
    color = dict()
    queue = deque()
    for vertex in range(1, vertex_count+1):
        if vertex in color:
            continue

        color[vertex] = False

        queue.append(vertex)
        while queue:
            vertex = queue.popleft()

            for connectee in edges.get(vertex, set()):
                if color.get(connectee) is color[vertex]:
                    return True

                if color.get(connectee) is None:
                    queue.append(connectee)
                    color[connectee] = not color[vertex]


    # -- return OK
    return False


def main():
    for _ in range(int(input())):
        print('NO' if tick() else 'YES')


if __name__ == '__main__':
    main()
