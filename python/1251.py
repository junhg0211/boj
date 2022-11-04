word = input()

s1 = word[1:-2].index(min(word[1:-2])) + 2

part = word[s1:]

s2 = part[1:-1].index(min(part[1:-1])) + 2 + s1

print(word[:s1][::-1] + word[s1:s2][::-1] + word[s2:][::-1])

