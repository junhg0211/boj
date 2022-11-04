from sys import stdin

factorials = [1, 1]


def factorial(n):
    if n < len(factorials):
        return factorials[n]
    if n == len(factorials):
        factorials.append(factorials[-1] * n)
        return factorials[n]
    for i in range(len(factorials), n):
        factorial(i)
    return factorial(n)


def ncr(n, r):
    return factorial(n) // (factorial(r) * factorial(n-r))



for i in range(int(stdin.readline())):
    w, e = map(int, stdin.readline().split())
    print(ncr(e, w))

