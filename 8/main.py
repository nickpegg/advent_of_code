import uuid
from typing import List, Optional

class Node(object):
    def __init__(self, children=None, metadata=None):
        # type: (Optional[List[Node]], Optional[List[int]] -> None
        self.id = str(uuid.uuid4())

        if children is not None:
            self.children = children
        else:
            self.children = []

        if metadata is not None:
            self.metadata = metadata
        else:
            self.metadata = []

    def __eq__(self, other):
        return self.children == other.children and self.metadata == other.metadata

    def __repr__(self):
        return f"<Node metadata={self.metadata}, children={self.children}>"


def parse_nodes(filename):
    # type: (str) -> List[Node]
    nums = [int(n) for n in open(filename).read().split()]
    return parse_node(nums)


def parse_node(nums):
    # type: (List[int]) -> Optional[Node]
    """
    Recursively parse the next node from the given numbers
    """
    if len(nums) == 0:
        return None

    num_children = nums.pop(0)
    num_entries = nums.pop(0)
    n = Node()

    for i in range(num_children):
        n.children.append(parse_node(nums))

    for i in range(num_entries):
        n.metadata.append(nums.pop(0))

    return n


def node_value(node):
    # type: (Node) -> int
    value = 0
    if len(node.children) == 0:
        value = sum(node.metadata)
    else:
        for i in node.metadata:
            idx = i - 1
            if len(node.children) > idx:
                value += node_value(node.children[idx])

    return value


def solution1(root):
    # type: (Node) -> int
    metadata_sum = 0
    to_visit = [root]

    while len(to_visit) > 0:
        n = to_visit.pop(0)
        metadata_sum += sum(n.metadata)
        to_visit.extend(n.children)
    return metadata_sum


def solution2(root):
    # type: (Node) -> int
    return node_value(root)


def main():
    root = parse_nodes('input.txt')
    print(f"solution 1: {solution1(root)}")
    print(f"solution 2: {solution2(root)}")


if __name__ == '__main__':
    main()
