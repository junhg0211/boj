from collections import deque


def main():
    count = int(input())
    channels = list()
    for _ in range(count):
        channels.append(input())

    queue = deque()
    queue.append(("", 0, tuple(channels)))

    while True:
        log, selection, channels = queue.popleft()

        if channels[0] == "KBS1" and channels[1] == "KBS2":
            print(log)
            break

        if selection < count - 1:
            # 1
            queue.append((log + "1", selection + 1, channels))

            # 3
            new_channels = (
                channels[:selection]
                + channels[selection + 1 : selection + 2]
                + channels[selection : selection + 1]
                + channels[selection + 2 :]
            )
            queue.append((log + "3", selection + 1, new_channels))

        if selection > 0:
            # 2
            queue.append((log + "2", selection - 1, channels))

            # 4
            new_channels = (
                channels[: selection - 1]
                + channels[selection : selection + 1]
                + channels[selection - 1 : selection]
                + channels[selection + 1 :]
            )
            queue.append((log + "4", selection - 1, new_channels))


if __name__ == "__main__":
    main()
