def main():
    occupied_thresh, complete_thresh, _ = map(int, input().split())

    mode = 0
    # 0 - IDLE
    # 1 - 사용 대기중
    # 2 - 사용중
    # 3 - 완료 대기중
    time = 0
    for i, state in enumerate(map(int, input())):
        if mode == 0:
            if state:
                mode = 1
                time = 0
            continue

        if mode == 1:
            if not state:
                mode = 0
                continue

            time += 1
            if time >= occupied_thresh:
                mode = 2
            continue

        if mode == 2:
            if not state:
                mode = 3
                time = 0
            continue

        if mode == 3:
            if state:
                mode = 2
                continue

            time += 1
            if time >= complete_thresh:
                mode = 0
                print(i+1)
            continue


if __name__ == '__main__':
    main()
