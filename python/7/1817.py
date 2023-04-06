def main():
    books, boxmax = map(int, input().split())

    if books == 0:
        print('0')
        return

    result = 1
    now_weight = 0
    for weight in map(int, input().split()):
        if now_weight + weight > boxmax:
            now_weight = weight
            result += 1
        else:
            now_weight += weight

    print(result)


if __name__ == '__main__':
    main()
