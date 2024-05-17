def main():
    song_count, question_count = map(int, input().split())

    time = 0
    singing = list()
    for i in range(song_count):
        duration = int(input())

        singing.append((i, duration, time))
        time += duration

    for _ in range(question_count):
        time = int(input())

        for i in range(song_count):
            index, duration, start_time = singing[i]
            if start_time <= time < start_time + duration:
                print(index + 1)
                break


if __name__ == "__main__":
    main()
