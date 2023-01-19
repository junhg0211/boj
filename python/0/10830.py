def matmul(a, b, size):
    result = list()
    for i in range(size):
        row = list()
        for j in range(size):
            value = 0
            for k in range(size):
                value += a[i][k] * b[k][j]
            value %= 1000
            row.append(value)
        result.append(row)
    return result


def main():
    size, power = map(int, input().split())

    matrix = list()
    for _ in range(size):
        matrix.append(list(map(int, input().split())))

    i = [[matrix[i][j] for j in range(size)] for i in range(size)]
    result = [[int(i == j) for j in range(size)] for i in range(size)]

    while power:
        if power & 1:
            # print(result, i, power)
            result = matmul(result, i, size)
        power >>= 1
        i = matmul(i, i, size)

    for row in result:
        print(' '.join(map(str, row)))


if __name__ == '__main__':
    main()
