def main():
    j = 1
    while True:
        size = int(input())

        if not size:
            return

        numbers = list()
        board = list()
        been = set()
        sum_ = None
        magic = True
        for _ in range(size):
            row = list()
            for number in map(int, input().split()):
                row.append(number)
                numbers.append(number)
                been.add(number)
            row_sum = sum(row)
            if sum_ is None:
                sum_ = row_sum
            elif sum_ != row_sum:
                magic = False
            board.append(row)

        if magic:
            for i in range(len(board)):
                column_sum = sum(map(lambda x: x[i], board))
                if column_sum != sum_:
                    magic = False
                    break

        diagonal = True
        d1, d2 = 0, 0
        for i, row in enumerate(board):
            d1 += row[i]
            d2 += row[-i-1]
        diagonal = d1 == d2 == sum_

        distinct = len(board)**2 == len(been)

        consecutive = True
        numbers.sort()
        for i in range(len(numbers) - 1):
            if numbers[i+1] - numbers[i] != 1:
                consecutive = False

        if not magic:
            print(f'Square {j}: Not a Magick Square')
        elif diagonal and distinct and consecutive:
            print(f'Square {j}: Magically-Magick Square')
        elif diagonal and distinct:
            print(f'Square {j}: Strongly-Magick Square')
        elif diagonal:
            print(f'Square {j}: Weakly-Magick Square')
        else:
            print(f'Square {j}: Semi-Magick Square')

        j += 1


if __name__ == '__main__':
    main()
