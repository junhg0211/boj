from collections import deque
from sys import stdin

input = stdin.readline

elements = {
    'cd', 'lr', 'cf', 'v', 'np', 'be', 'lv', 'o',
    'ba', 'lu', 'ca', 'i', 'sr', 'hg', 'tc', 'ru',
    'xe', 'pa', 'cu', 'rg', 'ac', 'db', 'he', 'nd',
    'na', 'cm', 'zr', 'mn', 'ag', 'kr', 'ar', 'sg',
    'u', 'cl', 'at', 'rh', 'mo', 'te', 'ce', 'ge',
    'la', 'ir', 'pd', 'eu', 'bk', 'au', 'pb', 'es',
    'os', 'fl', 'y', 'cn', 'fr', 'rn', 'tb', 'sc',
    'bi', 'cs', 'hs', 'nb', 'th', 'as', 'sn', 'b',
    'am', 'ti', 'zn', 'md', 'mt', 'in', 'pm', 'fm',
    'w', 'ra', 'rf', 'pu', 'pt', 'ds', 'hf', 'li',
    'si', 'po', 'c', 'fe', 'p', 's', 'n', 'k',
    'bh', 'ho', 'ne', 'yb', 'ni', 'br', 'cr', 'ga',
    'no', 'tl', 'rb', 'dy', 'f', 'al', 'sb', 'sm',
    're', 'co', 'ta', 'se', 'tm', 'er', 'gd', 'h',
    'pr', 'mg'
}


def tick():
    word = input().rstrip()
    indeces = deque([0])
    been = set()

    while indeces:
        # print(indeces)
        i = indeces.popleft()

        if i == len(word):
            print('YES')
            return

        if len(word) - i >= 2 \
                and word[i:i+2] in elements \
                and i+2 not in been:
            indeces.append(i+2)
            been.add(i+2)
        if word[i] in elements and i+1 not in been:
            indeces.append(i+1)
            been.add(i+1)

    print('NO')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
