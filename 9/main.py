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


class Circle(object):
    """
    Ring of marbles
    """
    def __init__(self):
        # type: () -> None
        self.marbles = blist()   # type: blist[int]
        self._current_idx = 0

    def add(self, marble):
        # type: (int) -> List[int]
        """
        :param marble: Marble to add to the circle
        :returns: list of marbles taken from circle
        """
        taken = []  # type: List[int]

        if len(self.marbles) == 0:
            self.marbles = [marble]
            return []

        if marble % 23 == 0:
            taken.append(marble)
            new_idx = self._current_idx - 7
            while new_idx < 0:
                new_idx += len(self.marbles)

            taken.append(self.marbles.pop(new_idx))
            if new_idx == len(self.marbles):
                new_idx = 0
        else:
            new_idx = self._current_idx + 2

            # Wrap the index around
            while new_idx > len(self.marbles):
                new_idx -= len(self.marbles)

            self.marbles.insert(new_idx, marble)

        self._current_idx = new_idx
        # self.trim()
        return taken

    def trim(self):
        # type: () -> None
        """
        Trim the circle of marbles we're not going to touch
        """
        if len(self.marbles) > 1000:
            print(self._current_idx)
            assert self._current_idx > 10
            self.marbles = self.marbles[10:]
            self._current_idx -= 10


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
        pos = float(self._circle._current_idx) / len(self._circle.marbles) * 100
        pos = int(pos)
        print(f"circle pos: {pos}%")



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
