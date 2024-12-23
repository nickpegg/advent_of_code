#!/usr/bin/env python

import re
from functools import total_ordering
from typing import Dict, List, Optional, Set, Tuple


MAX_TICKS = 100000


@total_ordering
class Step(object):
    def __init__(self, letter):
        # type: (str) -> None
        self.letter = letter
        self.children = []  # type: List[Step]
        self.parents = []   # type: List[Step]

    def __eq__(self, other):
        return self.letter == other.letter

    def __lt__(self, other):
        return self.letter < other.letter

    def __repr__(self):
        # type: () -> str
        return f"<Step {self.letter}>"

    def add_child(self, child):
        # type: (Step) -> None
        self.children.append(child)
        child.parents.append(self)

    def add_parent(self, parent):
        # type: (Step) -> None
        self.parents.append(parent)
        parent.children.append(self)


def parse_line(line):
    # type: (str) -> Tuple[str, str]
    line_re = r'Step (\w+) must be finished before step (\w+) can begin'
    match = re.match(line_re, line)
    if not match:
        raise RuntimeError(f"Unable to to parse {line}")

    a, z = match.groups()
    return (a.lower(), z.lower())


def prepare_steps(input_file):
    # type: (str) -> Dict[str, Step]
    steps = {}  # type: Dict[str, Step]

    with open(input_file) as f:
        for line in f.readlines():
            a, z = parse_line(line)

            for letter in (a, z):
                if letter not in steps:
                    steps[letter] = Step(letter)

            steps[a].add_child(steps[z])

    return steps


def find_roots(steps):
    # type: (Dict[str, Step]) -> List[Step]
    roots = []
    for step in steps.values():
        if not step.parents:
            roots.append(step)
    return sorted(roots)


def steps_to_take(steps):
    # type: (Dict[str, Step]) -> List[Step]
    # Build the list of steps to take
    to_take = []
    visited = set()     # type: Set[str]
    next_possible = find_roots(steps)
    while len(next_possible) > 0:
        next_step = next_possible.pop(0)
        if next_step.letter in visited:
            continue

        # Check that we're able to visit this step
        parent_letters = {p.letter for p in next_step.parents}
        if parent_letters - visited:
            # Still need a prereq, keep going
            continue

        to_take.append(next_step)
        visited.add(next_step.letter)

        for child in next_step.children:
            if child.letter not in visited:
                next_possible.append(child)

        next_possible = sorted(next_possible)

    return to_take


def solution1(steps):
    to_take = steps_to_take(steps)
    return ''.join((s.letter for s in to_take))


def solution2(steps, base_seconds=60, pool_size=5):
    # type: (Dict[str, Step], int, int) -> int
    # wrong answers:
    # - 425 (too low)
    # - 984 (too low)
    # - 985 !
    work_list = steps_to_take(steps)
    pool = WorkerPool(size=pool_size, time_offset=base_seconds)
    pool.schedule_all(work_list)
    pool.join()
    return pool.tick


class Worker(object):
    def __init__(self, tick=0, time_offset=60):
        # type: (int, int) -> None
        self.tick = tick
        self.time_offset = time_offset

        self.current_task = None    # type: Optional[Step]
        self.start_tick = -1

    def __repr__(self):
        # type () -> str
        return f"<Worker: {self.current_task}>"

    def start(self, task):
        # type (Step) -> None
        self.current_task = task
        self.start_tick = self.tick

    def do_tick(self):
        # type: () -> Optional[Step]
        did = None
        if self.current_task is not None:
            duration = time_for_task(
                self.current_task.letter, base_seconds=self.time_offset
            )
            if self.tick >= self.start_tick + duration - 1:
                # Was working, but finished task on this tick
                did = self.current_task
                self.current_task = None
                self.start_tick = -1

        self.tick += 1

        return did


class WorkerPool(object):
    def __init__(self, size=5, time_offset=60):
        # type: (int, int) -> None
        self.workers = [Worker(time_offset=time_offset) for i in range(size)]
        self.tick = 0

        self.done_tasks = []    # type: List[Step]

    @property
    def free_workers(self):
        # type: () -> List[Worker]
        return [w for w in self.workers if w.current_task is None]

    @property
    def busy_workers(self):
        # type: () -> List[Worker]
        return [w for w in self.workers if w.current_task is not None]

    def do_tick(self):
        # type: () -> List[Step]
        print(f"Tick {self.tick}, {self.workers}")
        done_tasks = []
        self.tick += 1
        for worker in self.workers:
            done_task = worker.do_tick()
            if done_task is not None:
                done_tasks.append(done_task)
            assert self.tick == worker.tick

        if self.tick == MAX_TICKS:
            raise RuntimeError(f"Timeout after {MAX_TICKS} ticks")

        self.done_tasks.extend(done_tasks)
        return done_tasks

    def schedule(self, task):
        # type: (Step) -> None
        """
        Try to run ``task``. If no worker is available, tick until one does
        """
        while len(self.free_workers) == 0:
            self.do_tick()
        worker = self.free_workers[0]
        worker.start(task)

    def schedule_all(self, tasks):
        # type: (List[Step]) -> None
        tasks = tasks.copy()

        while len(tasks) > 0:
            # Find the next workable task, may not be the first one!
            next_task = None
            for i in range(len(tasks)):
                prereqs = {p.letter for p in tasks[i].parents}
                completed = {t.letter for t in self.done_tasks}
                if len(prereqs - completed) == 0:
                    next_task = tasks.pop(i)
                    break

            if next_task is not None:
                self.schedule(next_task)
            else:
                if len(self.busy_workers) > 0:
                    self.do_tick()
                else:
                    raise RuntimeError(
                        "Deadlock! Work left to do, but prereqs aren't done "
                        "and workers are all free"
                    )


    def join(self):
        # type: () -> None
        """
        Tick until all workers are done
        """
        while len(self.busy_workers) > 0:
            self.do_tick()


def time_for_task(letter, base_seconds=60):
    # type: (str, int) -> int
    return ord(letter) - 96 + base_seconds


def main():
    steps = prepare_steps('input.txt')
    print(solution1(steps).upper())
    print(solution2(steps))


if __name__ == '__main__':
    main()
