until = int(input())
a, b = 1, 1

for _ in range(until):
    a, b = b, a + b

print(a % 10007)

