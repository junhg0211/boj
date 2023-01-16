def main():
    count, limit = map(int, input().split())
    dp = [[0 for _ in range(limit+1)] for _ in range(count)]

    for i in range(count):
        weight, score = map(int, input().split())

        for j in range(0, limit+1):
            if j < weight:
                dp[i][j] = dp[i-1][j]
            else:
                dp[i][j] = max(dp[i-1][j], dp[i-1][j-weight] + score)

    '''
    from pprint import pprint
    pprint(dp)
    '''

    print(dp[count-1][limit])


if __name__ == '__main__':
    main()
