def get_writes(letter):
    return (3,2,1,2,3,3,2,3,3,2,2,1,2,2,1,2,2,2,1,2,1,1,1,2,2,1)[ord(letter)-ord('A')]


a = input()
b = input()

string = ''
for i in range(len(a)):
    string += a[i] + b[i]

result = [get_writes(letter) for letter in string]

while len(result) > 2:
    for i in range(len(result) - 1):
        result[i] = (result[i] + result[i+1]) % 10
    result.pop()

print(result[0] * 10 + result[1])

