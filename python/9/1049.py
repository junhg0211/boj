from math import inf


def brand_price(number, six_price, one_price):
    price = 0

    while number >= 6:
        price += min(six_price, one_price * 6)
        number -= 6
        continue

    price += min(six_price, one_price * number)

    return price


def main():
    string_count, brand_count = map(int, input().split())

    min_price = [inf, inf]
    for _ in range(brand_count):
        prices = tuple(map(int, input().split()))
        min_price[0] = min(min_price[0], prices[0])
        min_price[1] = min(min_price[1], prices[1])

    print(brand_price(string_count, *min_price))


if __name__ == '__main__':
    main()
