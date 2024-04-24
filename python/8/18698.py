def main():
    count = int(input())

    for _ in range(count):
        steps = list(input())
        step_count = 0
        while steps and steps[0] == "U":
            steps.pop(0)
            step_count += 1

        print(step_count)


if __name__ == "__main__":
    main()
