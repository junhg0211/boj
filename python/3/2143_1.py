def binary_search(list_, value) -> bool:
    start = 0
    end = len(list_)

    while end - start > 1:
        mid = (start + end) // 2

        if list_[mid] == value:
            return True

        if list_[mid] < value:
            start = mid
        else:
            end = mid

    return False


def main():
    target_number = int(input())

    # -- get lists prefix sum
    a_length = int(input())
    a_prefix_sum = list()
    for number in map(int, input().split()):
        if not a_prefix_sum:
            a_prefix_sum.append(number)
            continue
        a_prefix_sum.append(a_prefix_sum[-1] + number)

    b_length = int(input())
    b_prefix_sum = list()
    for number in map(int, input().split()):
        if not b_prefix_sum:
            b_prefix_sum.append(number)
            continue
        b_prefix_sum.append(b_prefix_sum[-1] + number)

    # -- get counts and possible sums
    a_sums = set()
    a_sums_count = dict()
    for i in range(-1, a_length):
        for j in range(i+1, a_length):
            this = a_prefix_sum[j] - (0 if i == -1 else a_prefix_sum[i])
            a_sums.add(this)
            a_sums_count[this] = a_sums_count.get(this, 0) + 1

    b_sums = set()
    b_sums_count = dict()
    for i in range(-1, b_length):
        for j in range(i+1, b_length):
            this = b_prefix_sum[j] - (0 if i == -1 else b_prefix_sum[i])
            b_sums.add(this)
            b_sums_count[this] = b_sums_count.get(this, 0) + 1

    # -- count them up
    result = 0
    for a_sum in a_sums:
        b_sum = target_number - a_sum

        delta = a_sums_count[a_sum] * b_sums_count.get(b_sum, 0)
        result += delta
        # print(delta, a_sum, b_sum)

    print(result)


if __name__ == '__main__':
    main()
