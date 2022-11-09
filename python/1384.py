def main():
    group_number = 1
    while True:
        count = int(input())

        if count == 0:
            break

        names = list()
        negatives = list()
        for i in range(count):
            paper = input().split()
            names.append(paper.pop(0))
            for j, response in enumerate(paper):
                if response == 'N':
                    negatives.append((i-(j+1), i))

        print(f'Group {group_number}')
        if negatives:
            for subject, object_ in negatives:
                subject = names[subject % count]
                object_ = names[object_ % count]
                print(f'{subject} was nasty about {object_}')
        else:
            print('Nobody was nasty')

        print()

        group_number += 1


if __name__ == '__main__':
    main()
