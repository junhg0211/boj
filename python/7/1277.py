from sys import stdin
from heapq import heappop, heappush
from math import hypot, inf

input = stdin.readline


def main():
    # -- get variables
    plant_count, wire_count = map(int, input().split())

    length_limit = float(input())

    plant_position = dict()
    for i in range(1, plant_count+1):
        x, y = map(int, input().split())
        plant_position[i] = (x, y)

    wires = dict()
    for _ in range(wire_count):
        i, j = map(int, input().split())

        if i not in wires:
            wires[i] = set()
        wires[i].add(j)

        if j not in wires:
            wires[j] = set()
        wires[j].add(i)

    # -- get connectable table

    # 틀리면 여기에다가 이미 연결된 와이어들도 추가하기
    connectable = dict()
    for i in range(1, plant_count+1):
        for j in range(i+1, plant_count+1):
            dx = plant_position[i][0] - plant_position[j][0]
            dy = plant_position[i][1] - plant_position[j][1]
            distance = hypot(dx, dy)

            '''
            if distance > length_limit:
                continue
                '''

            if i not in connectable:
                connectable[i] = set()
            connectable[i].add((j, distance))

            if j not in connectable:
                connectable[j] = set()
            connectable[j].add((i, distance))

    # -- connect: dijkstra
    beens = set()
    heap = list()
    distances = dict()

    heappush(heap, (0.0, 1))
    distances[1] = 0.0

    while heap:
        distance, destination = heappop(heap)

        if destination == plant_count:
            print(int(distance * 1000))
            return

        if destination in beens:
            continue

        for to, length in connectable[destination]:
            if to in wires.get(destination, set()):
                heappush(heap, (distance, to))
                distances[to] = min(distances.get(to, inf), distance)
                continue

            heappush(heap, (distance + length, to))
            distances[to] = min(distances.get(to, inf), distance + length)

        beens.add(destination)


if __name__ == '__main__':
    main()
