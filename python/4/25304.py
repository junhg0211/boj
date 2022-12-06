total = int(input())

for _ in range(int(input())):
    price, count = map(int, input().split())
    total -= price * count

print('No' if total else 'Yes')
