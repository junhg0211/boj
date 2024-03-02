from math import inf


def get_hexagonal_number(n: int):
    return n * (2 * n - 1)


hexagonal_numbers = list()


dp = dict()


def main():
    target = int(input())

    dp = dict()
    i = 1
    while True:
        number = get_hexagonal_number(i)
        hexagonal_numbers.append(number)
        dp[number] = 1

        if hexagonal_numbers[-1] >= target:
            break

        i += 1

    stack = list()
    stack.append(target)
    got = set()
    while stack:
        number = stack.pop()
        print(number)

        if number in dp:
            if number == target:
                break
            continue

        good = True
        value = inf
        bads = list()
        for hexagonal_number in hexagonal_numbers:
            if hexagonal_number >= number:
                break

            that = number - hexagonal_number
            if that not in dp:
                if that not in got:
                    bads.append(that)
                    got.add(that)
                good = False
            else:
                value = min(dp[that] + 1, value)

        if good:
            dp[number] = value
        else:
            stack.append(number)
            stack.extend(bads)

    print(dp[target])


if __name__ == '__main__':
    main()
