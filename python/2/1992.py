from copy import copy
from math import log2


class Node:
    @staticmethod
    def structify(flattened) -> 'Node':
        return Node(flattened)

    def __init__(self, color):
        self.color = color

    def __eq__(self, other):
        if isinstance(other, Node):
            return all((self.color == other.color,))

        return False

    def __copy__(self):
        return Node(self.color)

    def flatten(self):
        return str(self.color)

    def to_quadtree(self) -> 'QuadTree':
        return QuadTree([copy(self) for _ in range(4)])


class QuadTree:
    @staticmethod
    def structify(flattened) -> 'QuadTree':
        children = list()
        for raw_child in flattened:
            children.append(structify(raw_child))
        return QuadTree(children)

    def __init__(self, children: list):
        self.children = children

    def flatten(self) -> list:
        return '(' + ''.join(map(lambda x: x.flatten(), self.children)) + ')'

    def fill(self, address: list, node: Node):
        address = copy(address)
        index = address.pop(0)

        if not len(address):
            self.children[index] = node
            return

        child = self.children[index]
        if not isinstance(child, QuadTree):
            self.children[index] = child.to_quadtree()
        self.children[index].fill(address, node)

        if self.children[index].is_having_same_children():
            self.children[index] = self.children[index].children[0]

    def is_having_same_children(self) -> bool:
        for i in range(3):
            if self.children[i] != self.children[3]:
                return False
        return True


def structify(flattened):
    if isinstance(flattened, str):
        return Node.structify(flattened)
    else:
        return QuadTree.structify(flattened)


def get_address(x: float, y: float, depth: int) -> list:
    unit = 2 ** (-depth)
    x = int(x / unit)
    y = int(y / unit)

    result = list()
    for _ in range(depth):
        result.insert(0, 2 * (y & 0x1) + (x & 0x1))
        x >>= 1
        y >>= 1

    return result


def main():
    size = int(input())
    tree = QuadTree([Node(0), Node(0), Node(0), Node(0)])
    for y in range(size):
        line = list(map(int, input()))
        for x in range(size):
            address = get_address(x/size, y/size, int(log2(size)))
            tree.fill(address, Node(line[x]))
    result = tree.flatten()
    if result == '(0000)': print(0)
    elif result == '(1111)': print(1)
    else: print(result)


if __name__ == '__main__':
    main()
