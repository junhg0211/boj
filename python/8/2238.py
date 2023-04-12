from sys import stdin

input = stdin.readline


def main():
    max_score, count = map(int, input().split())

    prices = dict()

    for i in range(count):
        name, price = input().split()
        price = int(price)

        if price in prices:
            prices[price].append((i, name))
        else:
            prices[price] = [(i, name)]

    result = min(prices.items(), key=lambda x: (len(x[1]), x[0]))

    print(result[1][0][1], result[0])


if __name__ == '__main__':
    main()
