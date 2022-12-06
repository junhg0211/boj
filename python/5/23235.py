def numbers():
    i = 0
    while True:
        i += 1
        yield i


def main():
    for i in numbers():
        if input() == '0':
            break

        print(f'Case {i}: Sorting... done!')


if __name__ == '__main__':
    main()
