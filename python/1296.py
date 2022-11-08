def probability(yeondu: str, name: str):
    name += yeondu

    l = name.count('L')
    o = name.count('O')
    v = name.count('V')
    e = name.count('E')

    result = (l+o) * (l+v) * (l+e) * (o+v) * (o+e) * (v+e)

    return result % 100


def main():
    yeondu = input()

    max_probability = 0
    max_team = 'a'

    for _ in range(int(input())):
        name = input()
        prob = probability(yeondu, name)
        if prob > max_probability:
            max_probability = prob
            max_team = name
        elif prob == max_probability:
            max_team = min(max_team, name)

    print(max_team)


if __name__ == '__main__':
    main()
