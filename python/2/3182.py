def get_popularity(connection, number):
    result = 0
    been = set()
    while number not in been:
        been.add(number)
        result += 1
        number = connection[number]

    return result


def main():
    n = int(input())

    connection = dict()

    for i in range(n):
        connection[i] = int(input())-1

    max_popularity = 0
    populariter = -1
    for i in range(n):
        popularity = get_popularity(connection, i)
        # print(i, popularity)
        if popularity > max_popularity:
            max_popularity = popularity
            populariter = i

    print(populariter + 1)


if __name__ == '__main__':
    main()
