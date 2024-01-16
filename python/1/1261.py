from heapq import heappush, heappop
from math import inf


def main():
    # -- get board
    width, height = map(int, input().split())

    board = list()
    for _ in range(height):
        row = list(map(int, input()))
        board.append(row)

    # -- dijkstra
    distances = dict()
    heap = list()
    beens = set()

    distances[(0, 0)] = 0
    heappush(heap, (0, 0, 0))

    while heap:
        weight, i, j = heappop(heap)

        if i == height-1 and j == width-1:
            print(weight)
            return

        if (i, j) in beens:
            continue

        if i > 0 and (i-1, j) not in beens:
            new_weight = weight + board[i-1][j]
            distances[(i-1, j)] = min(distances.get((i-1, j), inf), new_weight)
            heappush(heap, (new_weight, i-1, j))

        if j > 0 and (i, j-1) not in beens:
            new_weight = weight + board[i][j-1]
            distances[(i, j-1)] = min(distances.get((i, j-1), inf), new_weight)
            heappush(heap, (new_weight, i, j-1))

        if i < height - 1 and (i+1, j) not in beens:
            new_weight = weight + board[i+1][j]
            distances[(i+1, j)] = min(distances.get((i+1, j), inf), new_weight)
            heappush(heap, (new_weight, i+1, j))

        if j < width - 1 and (i, j+1) not in beens:
            new_weight = weight + board[i][j+1]
            distances[(i, j+1)] = min(distances.get((i, j+1), inf), new_weight)
            heappush(heap, (new_weight, i, j+1))

        beens.add((i, j))


if __name__ == '__main__':
    main()
