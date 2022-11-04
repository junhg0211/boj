coin_count, target = map(int, input().split())
coins = [int(input()) for _ in range(coin_count)]


def get_index(range_):
    for i, coin in enumerate(coins):
        if coin > range_:
            return i - 1
    return coin_count - 1


count = 0
while target:
    greatest_less = coins[get_index(target)]
    count += target // greatest_less
    target %= greatest_less

print(count)

