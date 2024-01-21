a = int(input())
b = int(input())
c = int(input())
d = int(input())

one = a == 8 or a == 9
two = d == 8 or d == 9
thr = b == c

print('ignore' if one and two and thr else 'answer')
