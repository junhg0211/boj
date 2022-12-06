cache = {1: 0}
target = int(input())

for i in range(2, target + 1):
    numbers = [i-1]
    if i % 3 == 0:
        numbers.append(i // 3)
    if i % 2 == 0:
        numbers.append(i // 2)
    cache[i] = min(map(lambda x: cache[x], numbers)) + 1

print(cache[target])

