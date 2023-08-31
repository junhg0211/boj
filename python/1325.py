from collections import deque
from sys import stdin

input = stdin.readline


def main():
    # --- get connections
    computer_count, connection_count = map(int, input().split())
    trust = dict()

    for _ in range(connection_count):
        a, b = map(int, input().split())

        if b not in trust:
            trust[b] = set()

        trust[b].add(a)

    # print(trust)

    infecteds = set()
    max_infected = 0
    for i in range(1, computer_count+1):
        infected = set()
        queue = deque([i])

        while queue:
            now = queue.popleft()
            infected.add(now)

            for connectee in trust.get(now, ()):
                if connectee in infected:
                    continue

                queue.append(connectee)

        if len(infected) > max_infected:
            max_infected = len(infected)
            infecteds.clear()

        if len(infected) == max_infected:
            infecteds.add(i)

    print(' '.join(map(str, sorted(infecteds))))


if __name__ == '__main__':
    main()
