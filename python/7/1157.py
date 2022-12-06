word = input().upper()

frequency = dict()

for character in word:
    if character in frequency:
        frequency[character] += 1
    else:
        frequency[character] = 1


max_frequency = 0
max_character = ''

for character in frequency:
    if frequency[character] > max_frequency:
        max_frequency = frequency[character]
        max_character = character

    elif max_frequency == frequency[character]:
        max_character = ''

if max_character:
    print(max_character)
else:
    print('?')

