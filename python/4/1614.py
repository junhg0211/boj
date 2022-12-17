def main():
    finger = int(input())
    count = int(input())

    divisor = (1 if finger in (1, 5) else 2)
    tmp = count // divisor
    result = 8 * tmp
    count -= tmp * divisor

    if count == 0:
        result += finger - 1
    else:
        result += 9 - finger

    print(result)


if __name__ == '__main__':
    main()
