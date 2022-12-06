from collections import deque


def exclude_connection(connection, start_node):
    if start_node not in connection:
        return

    queue = deque([start_node])

    while queue:
        now = queue.popleft()
        if now not in connection:
            continue
        connecteds = connection.pop(now)
        for connected in connecteds:
            if connected == start_node:
                continue
            if connected in connection:
                queue.append(connected)


def main():
    nodes, edges = map(int, input().split())
    connection = {i: [i] for i in range(1, nodes+1)}

    for _ in range(edges):
        a, b = map(int, input().split())

        connection[a].append(b)
        connection[b].append(a)

    component_count = 0
    while connection:
        i = next(iter(connection.keys()))
        exclude_connection(connection, i)
        component_count += 1

    print(component_count)


if __name__ == '__main__':
    main()
