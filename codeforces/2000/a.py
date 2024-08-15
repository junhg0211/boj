import re

count = int(input())
for _ in range(count):
    w = re.compile(r"^10\d+$")

    i = input()
    if w.match(i) is not None and int(i[2:]) >= 2 and i[2] != "0":
        print("YES")
    else:
        print("NO")
