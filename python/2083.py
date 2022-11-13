def main():
    while (line := input()) != '# 0 0':
        name, age, weight = line.split()
        age, weight = int(age), int(weight)

        print(name, end=' ')
        if age > 17 or weight >= 80:
            print('Senior')
        else:
            print('Junior')


if __name__ == '__main__':
    main()
