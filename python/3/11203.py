def get_path_number(directory):
    result = 0
    if directory[-1] == 'R':
        result = 1
    directory = '1' + directory[:-1].replace('L', '0').replace('R', '1')
    return result + int(directory, 2)


def main():
    line = input().split(' ')

    height = int(line[0])
    if len(line) > 1:
        directory = line[1]
    else:
        directory = ''

    root = 2**(height+1) - 1

    path = ''
    for direction in directory:
        path += direction
        root -= get_path_number(path)

    print(root)


if __name__ == '__main__':
    main()
