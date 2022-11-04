h, m = input().split()
h, m = int(h), int(m)

m -= 45

if m < 0:
    h -= 1
    m += 60
if h < 0:
    h += 24

print(h, m)

