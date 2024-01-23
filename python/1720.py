dp = dict()


def f(x) -> tuple[int, int, int]:
    if x in dp:
        return dp[x]

    i = sum(f(x-1))
    m = sum(f(x-2))
    o = f(x-2)[2]

    value = (m, i, o)
    dp[x] = value
    return value


def main():
    # hw: 12, 21, 22
    dp[1] = (0, 1, 0)
    dp[2] = (1, 1, 1)

    print(sum(f(int(input()))))


if __name__ == '__main__':
    main()


"""
1
I
I

2
II MM OO

3
III IMM IOO

4
IIII IIMM IIOO IMMI IOOI MMII MMMM MMOO OOOO

i = sum(f(x-1))
m = sum(f(x-2))
o = f(x-2)[2]
"""
