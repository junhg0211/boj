def main():
    music_count, music_length, duration = map(int, input().split())

    time = 0
    for _ in range(music_count):
        time += music_length

        if 0 < (time + 5) % duration <= 5:
            print(time + 5 - (time + 5) % duration)
            return

        time += 5

    print(time)


if __name__ == '__main__':
    main()
