def main():
    case = 1
    while True:
        student_count = int(input())

        if student_count == 0:
            break

        names = list()
        for _ in range(student_count):
            name = input()
            names.append([name, 2])

        for _ in range(student_count * 2 - 1):
            index, _ = input().split()
            index = int(index) - 1
            names[index][1] -= 1

        for name, count in names:
            if count == 1:
                print(case, name)
                break

        case += 1


if __name__ == '__main__':
    main()
