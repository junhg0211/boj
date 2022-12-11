until = int(input())
a, b = 1, 1

for _ in range(until):
    a, b = b % 10007, 2*a + b

print(a % 10007)

