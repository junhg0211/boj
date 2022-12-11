from sys import stdin

input = stdin.readline


class MaxHeap:
    def __init__(self):
        self.queue = list()

    def append(self, value):
        self.queue.append(value)
        index = self.get_last_index()
        parent_index = self.get_parent_index(index)

        while parent_index >= 0 and not self.check_valid_up(index, value):
            self.swap(index, parent_index)
            index = parent_index
            parent_index = self.get_parent_index(index)

    def swap(self, index: int, parent_index: int = -1):
        parent_index = self.get_parent_index(index) if parent_index != -1 else parent_index
        self.queue[parent_index], self.queue[index] = self.queue[index], self.queue[parent_index]
        return self.queue[parent_index]

    def check_valid_up(self, index: int, value=None):
        parent = self.queue[self.get_parent_index(index)]
        value = self.queue[index] if value is None else value
        return parent > value

    def get_last_index(self) -> int:
        return len(self.queue) - 1

    def get_parent_index(self, index: int):
        return (index-1) // 2

    def pop(self):
        if not self.queue:
            return 0

        last_index = self.get_last_index()
        self.swap(0, last_index)
        result = self.queue.pop()

        self.min_heapify(0)

        return result

    def min_heapify(self, index: int):
        left_child_index = self.get_left_index(index)
        right_child_index = self.get_right_index(index)

        minimal_index = index

        if left_child_index < len(self.queue) \
                and self.queue[left_child_index] > self.queue[minimal_index]:
            minimal_index = left_child_index
        if right_child_index < len(self.queue) \
                and self.queue[right_child_index] > self.queue[minimal_index]:
            minimal_index = right_child_index

        if minimal_index != index:
            self.swap(minimal_index, index)
            self.min_heapify(minimal_index)

    def get_left_index(self, index: int) -> int:
        return index * 2 + 1

    def get_right_index(self, index: int) -> int:
        return index * 2 + 2


def main():
    heap = MaxHeap()
    for _ in range(int(input())):
        value = int(input())

        if value:
            heap.append(value)
            continue

        print(heap.pop())


if __name__ == '__main__':
    main()
