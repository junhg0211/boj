def main():
    wow = input()

    if wow == '""' or wow == '"':
        print('CE')
    elif wow.startswith('"') and wow.endswith('"'):
        print(wow[1:-1])
    else:
        print('CE')


if __name__ == '__main__':
    main()
