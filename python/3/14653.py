from sys import stdin

input = lambda: stdin.readline().rstrip()


def main():
    person_count, message_count, message_index = map(int, input().split())

    readers = set()
    previous_number = -1
    for _ in range(message_index-1):
        count, name = input().split()
        count = int(count)

        if previous_number != count:
            previous_number = count
            readers.clear()
        readers.add(name)

    names = set(chr(i) for i in range(ord('B'), ord('B')+person_count-1))
    for i in range(message_count - message_index + 1):
        count, name = input().split()
        count = int(count)
        if name in names:
            names.remove(name)
        if i == 0 and previous_number == count:
            names -= readers
        if count == i == 0:
            names.clear()
            break

    if names:
        print(' '.join(sorted(names)))
    else:
        print('-1')


if __name__ == '__main__':
    main()
