def main():
    numbers = set()
    for i in range(int(input())):
        numbers.add(int(input()))

    result = 4
    for number in numbers:
        for j in range(-2, 3):
            need = 0
            for i in range(-2, 3):
                if number + j + i not in numbers:
                    need += 1

            if not (result := min(need, result)):
                break
        if not result:
            break

    print(result)


if __name__ == '__main__':
    main()
