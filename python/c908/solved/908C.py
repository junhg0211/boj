def main():
    count = int(input())

    result = 0
    for i in range(1, count+1):
        result += count / i

    print(result)


if __name__ == '__main__':
    main()
