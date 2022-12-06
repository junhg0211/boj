dc, bc = map(int, input().split())

ds = set(input() for _ in range(dc))
bs = set(input() for _ in range(bc))

dbs = ds & bs
print(len(dbs))
print('\n'.join(sorted(dbs)))

