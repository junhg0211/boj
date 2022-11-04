from sys import setrecursionlimit
from math import inf

setrecursionlimit(10**6)

number = int(input())
cache = dict()


def f(x):
    if x in cache:
        return cache[x]

    if int(x**0.5) == x**0.5:
        cache[x] = 1
        return 1

    min_ = inf
    for i in range(1, x//2 + 1):
        count = f(i) + f(x-i)
        if count < min_:
            min_ = count
    cache[x] = min_
    return min_


print(f(number))

