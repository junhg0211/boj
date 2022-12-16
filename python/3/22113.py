from sys import stdin

input = stdin.readline


def main():
    bus_count, take_count = map(int, input().split())
    takes = tuple(map(int, input().split()))

    transfer_rate = tuple(tuple(map(int, input().split())) for _ in range(bus_count))

    result = 0
    for i in range(take_count-1):
        from_ = takes[i]-1
        to = takes[i+1]-1

        result += transfer_rate[from_][to]

    print(result)


if __name__ == '__main__':
    main()
