def is_geummin(number: int):
    return not str(number).replace('7', '').replace('4', '')


def main():
    limit = int(input())

    for i in range(limit, 3, -1):
        if is_geummin(i):
            print(i)
            return


if __name__ == '__main__':
    main()
