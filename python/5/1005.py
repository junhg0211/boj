from sys import stdin, setrecursionlimit

setrecursionlimit(10**6)


class Building:
    def __init__(self, duration: int):
        self.duration = duration
        self.connections_from = list()
        self.total_duration = None

    def get_total_duration(self):
        if self.total_duration is not None:
            return self.total_duration

        if self.connections_from:
            self.total_duration = \
                max(from_.get_total_duration() for from_ in self.connections_from) \
                + self.duration
        else:
            self.total_duration = self.duration
        return self.total_duration


for i in range(int(stdin.readline())):
    n, k = map(int, stdin.readline().split())
    buildings = list(map(lambda x: Building(int(x)), stdin.readline().split()))
    for from_, to in (map(int, stdin.readline().split()) for _ in range(k)):
        from_ -= 1
        to -= 1
        buildings[to].connections_from.append(buildings[from_])
    aim = int(stdin.readline()) - 1

    print(buildings[aim].get_total_duration())

