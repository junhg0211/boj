def main():
    height = int(input())

    tree = list()
    for _ in range(height):
        tree.append(list(map(int, input().split())))

    for i in range(height-2, -1, -1):
        for j in range(0, len(tree[i])):
            tree[i][j] += max(tree[i+1][j], tree[i+1][j+1])

    print(tree[0][0])


if __name__ == '__main__':
    main()
