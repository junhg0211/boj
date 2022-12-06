def main():
    for _ in range(2):
        numbers = tuple(map(int, input().split()))
        print(numbers[0]*6 + numbers[1]*3 + numbers[2]*2 + numbers[3] + numbers[4]*2, end=' ')


if __name__ == '__main__':
    main()
