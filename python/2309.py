def main():
    heights = list()
    height_sum = 0
    for _ in range(9):
        height = int(input())
        heights.append(height)
        height_sum += height

    height_sum -= 100
    end = False
    for i in range(9):
        for j in range(i+1, 9):
            if heights[i] + heights[j] != height_sum:
                continue

            heights.pop(j)
            heights.pop(i)
            end = True
            break

        if end:
            break

    print('\n'.join(map(str, sorted(heights))))


if __name__ == '__main__':
    main()
