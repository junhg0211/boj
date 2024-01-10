def main():
    dojang = int(input())
    gagyeok = int(input())

    result = gagyeok
    if dojang >= 5:
        result = min(gagyeok - 500, result)
    if dojang >= 10:
        result = min(gagyeok * 0.9, result)
    if dojang >= 15:
        result = min(gagyeok - 2000, result)
    if dojang >= 20:
        result = min(gagyeok * 0.75, result)

    result = max(result, 0)

    print(int(result))



if __name__ == '__main__':
    main()
