def main():
    numbers = [0]
    min_diff = 100
    wow = 0
    for _ in range(10):
        number = int(input())
        real_number = numbers[-1] + number
        numbers.append(real_number)

        if (diff := abs(100 - real_number)) <= min_diff:
            wow = real_number
            min_diff = diff

    print(wow)


if __name__ == "__main__":
    main()
