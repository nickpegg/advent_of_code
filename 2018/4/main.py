import datetime
import enum
import re
from typing import Dict, List, Optional, Tuple


@enum.unique
class EventType(enum.IntEnum):
    BEGIN_SHIFT = 0
    SLEEP = 1
    WAKE = 2


class Event(object):
    def __init__(self, dt, _type, guard_id=None):
        # type: (datetime.datetime, EventType, Optional[int]) -> None
        self.dt = dt
        self.type = _type
        self.guard_id = guard_id

    def __repr__(self):
        # type: () -> str
        return '<Guard #{} does {} at {}>'.format(
            self.guard_id, self.type, self.dt
        )


class Guard(object):
    def __init__(self, _id):
        # type: (int) -> None
        self.id = _id
        self.events = []    # type: List[Event]

        # count of times they were asleep for a given minute
        self.minutes = {m: 0 for m in range(60)}    # type: Dict[int, int]

    def __repr__(self):
        # type: () -> str
        return '<Guard #{}>'.format(self.id)

    def add_event(self, event):
        # type: (Event) -> None
        self.events.append(event)
        if len(self.events) == 1:
            return

        # Calculate how long they were sleeping
        last_event = self.events[-2]
        if event.type == EventType.WAKE and last_event.type == EventType.SLEEP:
            for minute in range(last_event.dt.minute, event.dt.minute + 1):
                self.minutes[minute] += 1


    @property
    def minutes_asleep(self):
        # type: () -> int
        """
        Returns the total number of minutes asleep
        """
        num_asleep = 0
        for minute, count in self.minutes.items():
            num_asleep += count
        return num_asleep


def parse_logline(line):
    # type: (str) -> Event
    line_re = r'\[(.+?)\] (.+)'

    match = re.match(line_re, line)
    if not match:
        raise RuntimeError('Unable to parse log line: {}'.format(line))

    dt_string, message = match.groups()

    # parse the date and time
    dt = datetime.datetime.strptime(dt_string, '%Y-%m-%d %H:%M')

    # parse the message
    guard_id = None
    if message == 'falls asleep':
        event_type = EventType.SLEEP
    elif message == 'wakes up':
        event_type = EventType.WAKE
    elif 'begins shift' in message:
        event_type = EventType.BEGIN_SHIFT
        groups = re.findall(r'Guard #(\d+)', message)
        guard_id = int(groups[0])
    else:
        raise RuntimeError('Unable to get event type from: {}'.format(message))

    return Event(dt, event_type, guard_id=guard_id)


def load_events(path):
    # type: (str) -> List[Event]
    events = [] # type: List[Event]
    with open(path) as f:
        for line in f.readlines():
            events.append(parse_logline(line.strip()))

    # sort events by datetime
    events = sorted(events, key=lambda x: x.dt)
    return events


def load_guards(events):
    # type: (List[Event]) -> Dict[int, Guard]
    guards = {}     # type: Dict[int, Guard]

    # Add events to Guards. Events are assumed to be sorted chronologically.
    last_guard = None
    for event in events:
        if event.guard_id is not None:
            last_guard = event.guard_id

        if event.guard_id is None and last_guard is None:
            raise RuntimeError(
                'Got an event before knowing about any guard! {}'.format(event)
            )

        if event.guard_id is not None and event.guard_id not in guards:
            guards[event.guard_id] = Guard(event.guard_id)

        if last_guard is not None:
            guards[last_guard].add_event(event)

    return guards


def solution1(guards):
    # type: (Dict[int, Guard]) -> Tuple[Guard, int]

    # Invalid answers:
    # - 182913 (Guard 3209 on minute 57) - too high
    # - 21634 (Guard 373 on minute 58) - too low

    # Find the guard with the most time sleeping
    sleepiest = None
    for gid, guard in guards.items():
        if sleepiest is None or sleepiest.minutes_asleep < guard.minutes_asleep:
            sleepiest = guard
    if sleepiest is None:
        raise RuntimeError("Unable to find the sleepiest guard!")

    print('Sleepiest guard: {}'.format(sleepiest))

    # Find the minute they were sleeping the most
    best = {'minute': -1, 'count': 0}
    for minute, count in sleepiest.minutes.items():
        if best['count'] < count:
            best['count'] = count
            best['minute'] = minute

    if best['minute'] == -1:
        raise RuntimeError("Unable to determine sleepiest minute")

    print('They were sleeping the most on minute {}'.format(best['minute']))

    magic_num = sleepiest.id * best['minute']
    print('Magic number: {}'.format(magic_num))

    return (sleepiest, best['minute'])


def solution2(guards):
    # type: (Dict[int, Guard]) -> Tuple[Guard, int]
    longest_sleeper = None
    most_minute = None

    highest_count = 0
    for guard in guards.values():
        for minute, count in guard.minutes.items():
            if count > highest_count:
                highest_count = count
                longest_sleeper = guard
                most_minute = minute

    if longest_sleeper is None or most_minute is None:
        raise RuntimeError('Not able to find solution 2')

    print("Longest sleeper: {}".format(longest_sleeper))
    print("  slept most on minute {}".format(most_minute))

    magic_num = longest_sleeper.id * most_minute
    print("  magic number: {}".format(magic_num))

    return (longest_sleeper, most_minute)



def main():
    events = load_events('input.txt')
    guards = load_guards(events)
    solution1(guards)
    solution2(guards)


if __name__ == '__main__':
    main()
