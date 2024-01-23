from math import pi, ceil

dp = dict()


def f(x):
    if x <= pi:
        return 1

    if x in dp:
        return dp[x]

    '''
    if not isinstance(x, int):
        return f(x-1) + f(x-pi)
        '''

    dp[x] = (f(x-1) + f(x-pi)) % 1_000_000_000_000_000_000
    return dp[x]


def main():
    limit = int(input())
    for i in range(3, limit):
        f(i)

    print(f(limit))

'''
    from pprint import pprint
    pprint(sorted(dp.items()))
    '''


if __name__ == '__main__':
    main()
