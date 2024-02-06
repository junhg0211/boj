from math import log, sin, sqrt


results = [1]


def get(x):
    if x < len(results):
        return results[x]
    results.append(get(int(x - sqrt(x))) + get(int(log(x))) + get(int(x * sin(x)**2)))
    return results[x]


while True:
    index = int(input())
    if index == -1:
        break
    for i in range(len(results), index):
        get(i)

    print(get(index) % 1000000)

