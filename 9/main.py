from collections import deque
from typing import List, Tuple

from blist import blist


class Bag(object):
    """
    A bag of marbles
    """
    def __init__(self, highest):
        # type: (int) -> None
        self._marbles = deque(range(highest + 1))

    @property
    def empty(self):
        # type: () -> bool
        return len(self._marbles) == 0

    def get(self):
        # type: () -> int
        return self._marbles.popleft()

    def add(self, marble):
        # type: (int) -> None
        inserted = False

        for i in range(len(self._marbles)):
            if self._marbles[i] > marble:
                self._marbles.insert(i, marble)
                inserted = True
                break

        if not inserted:
            self._marbles.append(marble)


class Node(object):
    def __init__(self, value):
        self.value = value
        self.next = None
        self.prev = None

    def prepend(self, value):
        # type: (Any) -> None
        node = Node(value)
        node.next = self
        node.prev = self.prev

        if self.prev is not None:
            self.prev.next = node
        self.prev = node

    def append(self, value):
        # type: (Any) -> None
        node = Node(value)
        node.prev = self
        node.next = self.next

        if self.next is not None:
            self.next.prev = node
        self.next = node

    def delete(self):
        # type: () -> None
        """
        Remove this Node from the LinkedList
        """
        self.prev.next = self.next
        self.next.prev = self.prev


class Circle(object):
    """
    Ring of marbles
    """
    def __init__(self):
        # type: () -> None
        self.current_node = None
        self.head = None

    def add(self, marble):
        # type: (int) -> List[int]
        """
        :param marble: Marble to add to the circle
        :returns: list of marbles taken from circle
        """
        taken = []  # type: List[int]

        if self.current_node is None:
            self.current_node = Node(marble)
            self.current_node.next = self.current_node
            self.current_node.prev = self.current_node
            self.head = self.current_node
            return []

        if marble % 23 == 0:
            taken.append(marble)
            for i in range(6):
                self.current_node = self.current_node.prev

            to_take = self.current_node.prev
            to_take.delete()
            taken.append(to_take.value)
            del to_take
        else:
            # advance one
            self.current_node = self.current_node.next

            # add the marble after this one
            self.current_node.append(marble)

            # advance one more so we're pointing to the new marble
            self.current_node = self.current_node.next

        return taken

    def as_list(self):
        # type: () -> List[int]
        """
        Return the ring as a list of integers
        """
        da_list = []

        if self.head is None:
            return []

        cur = self.head
        da_list.append(cur.value)
        cur = self.head.next

        while cur != self.head:
            da_list.append(cur.value)
            cur = cur.next

        return da_list


class Game(object):
    def __init__(self, players, last_marble):
        # type: (int, int) -> None
        self.players = {player: 0 for player in range(players)}

        self._current_player = 0
        self._circle = Circle()
        self._bag = Bag(last_marble)

        self._turn = 0

    @property
    def high_score(self):
        # type: () -> Tuple[int, int]
        highest_score = 0
        best_player = None

        for player, score in self.players.items():
            if score > highest_score:
                highest_score = score
                best_player = player

        if best_player is None:
            raise RuntimeError("Unable to determine highest score")

        return (best_player, highest_score)

    def turn(self):
        # type: () -> None
        """
        Play a turn
        """
        self._turn += 1

        self._current_player += 1
        if self._current_player not in self.players:
            self._current_player = 0

        marble = self._bag.get()
        taken = self._circle.add(marble)

        pid = self._current_player
        self.players[pid] += sum(taken)

        # Output status every 1000 turns
        if self._turn % 1000 == 0:
            self.report()

    def run(self):
        # type: () -> None
        """
        Play turns until there are no more marbles
        """
        while not self._bag.empty:
            self.turn()
        self.report()

    def report(self):
        # type () -> None
        print(f"Turn {self._turn}, highest score: {self.high_score}, bag has {len(self._bag._marbles)} marbles left")



def solution1(players, last_marble):
    # type: (int, int) -> int
    game = Game(players, last_marble)
    game.run()

    _, score = game.high_score
    return score


def main():
    # type: () -> None
    players = 447
    last_marble = 71510
    x = solution1(players, last_marble)
    print(f"High score: {x}")

    x = solution1(players, last_marble * 100)
    print(f"High score: {x}")


if __name__ == '__main__':
    main()
