ns = (int(input()) for _ in range(3))
if sum(ns) == 180:
    print(('Equilateral','Isosceles','Scalene')[len(set(ns))])

